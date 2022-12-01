mod geometry_manager;
mod materials_manager;

use std::f32::consts::PI;

use bevy::prelude::*;
use doome_engine::{HEIGHT, WIDTH};
use doome_raytracer as rt;
use glam::{vec2, vec3, Vec4Swizzles};
use instant::Instant;

use self::geometry_manager::*;
use self::materials_manager::*;
use crate::assets::{AssetHandle, Assets, Model};
use crate::components::*;
use crate::convert::physical_to_graphical;
use crate::doome::DoomeRenderer;
use crate::physics::components::Collider;
use crate::renderer::RendererState;
use crate::rendering_options::RenderingOptions;

pub struct DoomeRaytracerPlugin;

#[derive(StageLabel)]
enum DoomeRaytracingStage {
    DeleteGeometry,
    Update,
    Render,
}

impl Plugin for DoomeRaytracerPlugin {
    fn build(&self, app: &mut App) {
        let state = {
            let camera = rt::Camera::new(
                Default::default(),
                Default::default(),
                vec3(0.0, -1.0, 0.0),
                1.0,
                vec2(WIDTH as _, HEIGHT as _),
                PI / 2.0,
            );

            State {
                geometry: Default::default(),
                camera,
                lights: Default::default(),
                materials: Default::default(),
            }
        };

        app.add_stage_after(
            CoreStage::Update,
            DoomeRaytracingStage::DeleteGeometry,
            SystemStage::single_threaded(),
        );

        app.add_stage_after(
            DoomeRaytracingStage::DeleteGeometry,
            DoomeRaytracingStage::Update,
            SystemStage::parallel(),
        );

        app.add_stage_after(
            DoomeRaytracingStage::Update,
            DoomeRaytracingStage::Render,
            SystemStage::single_threaded(),
        );

        app.insert_resource(state);

        app.add_system_to_stage(
            DoomeRaytracingStage::DeleteGeometry,
            sync_deleted_geometry,
        );
        app.add_system_to_stage(
            DoomeRaytracingStage::Update,
            sync_created_geometry,
        );
        app.add_system_to_stage(
            DoomeRaytracingStage::Update,
            sync_updated_geometry,
        );
        app.add_system_to_stage(
            DoomeRaytracingStage::Update,
            update_collider_data_for_debug_pass,
        );

        app.add_system_to_stage(DoomeRaytracingStage::Update, sync_lights);
        app.add_system_to_stage(DoomeRaytracingStage::Update, sync_camera);
        app.add_system_to_stage(DoomeRaytracingStage::Render, render);

        app.world.spawn(Camera::default());
    }
}

#[derive(Resource)]
struct State {
    geometry: GeometryManager,
    camera: rt::Camera,
    lights: rt::Lights,
    materials: MaterialsManager,
}

fn sync_deleted_geometry(
    mut state: ResMut<State>,
    removed_entities: RemovedComponents<Synced>,
) {
    for entity in removed_entities.iter() {
        state.geometry.free(entity);
        state.materials.free(entity);
    }
}

fn sync_created_geometry(
    mut commands: Commands,
    mut state: ResMut<State>,
    assets: Res<Assets>,
    models: Query<
        (
            Entity,
            &GeometryType,
            &AssetHandle<Model>,
            &Transform,
            Option<&Material>,
        ),
        Added<AssetHandle<Model>>,
    >,
) {
    let state = &mut *state;

    for (entity, &geo_type, model, &xform, mat) in models.iter() {
        let model = assets.model(*model);
        let xform = xform.compute_matrix();

        let mut mat = {
            let base_mat = model.material.materialize();

            mat.map(|mat| mat.merge_with(base_mat)).unwrap_or(base_mat)
        };

        if let Some(a) = &mut mat.alpha {
            *a = ease_in_ease_out(*a);
        }

        let mat_id = state.materials.alloc(entity, mat.materialize());
        let tex = mat.texture.map(|tex_id| assets.texture(tex_id));

        state
            .geometry
            .builder()
            .add_model(entity, geo_type, model, xform, mat, mat_id, tex);

        commands.entity(entity).insert(Synced);
    }
}

fn sync_updated_geometry(
    mut state: ResMut<State>,
    assets: Res<Assets>,
    models: Query<
        (
            Entity,
            &GeometryType,
            &AssetHandle<Model>,
            &Transform,
            Option<&Material>,
        ),
        Or<(
            Changed<Transform>,
            Changed<Material>,
            Changed<AssetHandle<Model>>,
        )>,
    >,
) {
    let state = &mut *state;
    let mut geo = state.geometry.updater();

    for (entity, &geo_type, model, &xform, mat) in models.iter() {
        if geo_type == GeometryType::Static {
            // TODO it's fine to overwrite materials for them
            continue;
        }

        let model = assets.model(*model);
        let xform = xform.compute_matrix();

        let mut mat = {
            let base_mat = model.material.materialize();

            mat.map(|mat| mat.merge_with(base_mat)).unwrap_or(base_mat)
        };

        if let Some(a) = &mut mat.alpha {
            *a = ease_in_ease_out(*a);
        }

        // TODO wasteful
        let mat_id = state.materials.alloc(entity, mat.materialize());
        let tex = mat.texture.map(|tex_id| assets.texture(tex_id));

        geo.update_model(entity, model, xform, mat, mat_id, tex);
    }
}

// TODO doing this each frame feels wonky
fn sync_lights(
    mut state: ResMut<State>,
    lights: Query<(&Light, &Transform, &Color, Option<&Visibility>)>,
) {
    state.lights = Default::default();

    let lights = lights
        .iter()
        .filter(|(_, _, _, vis)| vis.map_or(true, |vis| vis.is_visible))
        .filter(|(light, _, _, _)| light.enabled && light.intensity > 0.0);

    for (light, transform, color, _) in lights {
        let position = transform.translation;
        let intensity = ease_in_ease_out(light.intensity);

        match light.kind {
            LightKind::Point => {
                state.lights.push(rt::Light::point(
                    position,
                    color.into_vec3(),
                    intensity,
                ));
            }

            LightKind::Spot { point_at, angle } => {
                state.lights.push(rt::Light::spot(
                    position,
                    point_at,
                    angle,
                    color.into_vec3(),
                    intensity,
                ));
            }
        }
    }
}

fn sync_camera(
    rendering_options: Res<RenderingOptions>,
    mut state: ResMut<State>,
    renderer_state: Res<RendererState>,
    renderer: Res<DoomeRenderer>,
    camera: Query<&Camera, Changed<Camera>>,
) {
    let Ok(camera) = camera.get_single() else { return };

    state.camera.update(|origin, look_at, _| {
        *origin = camera.origin;
        *look_at = camera.look_at;
    });

    if rendering_options.debug_pass_enabled {
        let view_proj = {
            let offset = Vec3::NEG_Y * 2.0;
            let view = Mat4::look_at_rh(
                state.camera.origin.xyz() + offset,
                state.camera.look_at.xyz() + offset,
                state.camera.up.xyz(),
            );
            let fov = state.camera.viewport_size.z;
            let aspect =
                state.camera.viewport_size.x / state.camera.viewport_size.y;
            let proj = Mat4::perspective_rh(fov, aspect, 0.1, 100.0);
            proj * view
        };

        let queue = &renderer_state.queue;
        renderer.debug_pass.update_projection(queue, view_proj)
    }
}

fn update_collider_data_for_debug_pass(
    rendering_options: Res<RenderingOptions>,
    renderer: Res<DoomeRenderer>,
    renderer_state: Res<RendererState>,
    colliders: Query<(&Collider, &Transform)>,
) {
    if !rendering_options.debug_pass_enabled {
        return;
    }

    let queue = &renderer_state.queue;

    let mut lines = vec![];

    for (collider, transform) in colliders.iter() {
        let polygon = collider.to_polygon(transform);

        for (start, end) in polygon.iter_edges() {
            lines.push(physical_to_graphical(start));
            lines.push(physical_to_graphical(end));
        }
    }

    renderer
        .debug_pass
        .update_data(queue, &lines, lines.len() / 2);
}

fn render(
    time: Res<Time>,
    rendering_options: Res<RenderingOptions>,
    renderer: Res<DoomeRenderer>,
    renderer_state: Res<RendererState>,
    mut raytracer_state: ResMut<State>,
) {
    let Some(surface) = renderer_state.surface.as_ref() else {
        log::warn!("Surface not yet initialized");
        return;
    };

    let Ok(current_texture) = surface.get_current_texture() else { return };
    let device = &renderer_state.device;
    let queue = &renderer_state.queue;

    let intermediate_texture = &renderer.intermediate_output_texture_view;
    let sse_texture = &renderer.sse_output_texture_view;

    let tt = Instant::now();

    let DoomeRenderer {
        raytracer,
        pixels,
        screen_space_effects,
        debug_pass,
        scaler,
        shader_constants,
        ..
    } = &*renderer;

    let raytracer_state = &mut *raytracer_state;

    let Some((
        static_geo,
        static_geo_index,
        dynamic_geo,
        uvs
    )) = raytracer_state.geometry.inner() else { return };

    let texture_view = current_texture
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_command_encoder"),
        });

    shader_constants.write0(
        queue,
        &rt::ShaderConstants {
            width: WIDTH as f32,
            height: HEIGHT as f32,
            scaled_width: renderer.width,
            scaled_height: renderer.height,
            time: time.elapsed_seconds(),
            ..rt::ShaderConstants::default()
        },
    );

    raytracer.render(
        queue,
        &mut encoder,
        static_geo,
        static_geo_index,
        dynamic_geo,
        uvs,
        &raytracer_state.camera,
        &raytracer_state.lights,
        raytracer_state.materials.inner(),
        intermediate_texture,
    );

    pixels.render(queue, &mut encoder, shader_constants, intermediate_texture);

    if rendering_options.sse_enabled {
        screen_space_effects.render(
            queue,
            &mut encoder,
            shader_constants,
            sse_texture,
        );
    } else {
        // unfortunately we have to copy the intermediate_texture to the sse_texture
        // queue.write_texture(texture, data, data_layout, size)
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &renderer.intermediate_output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &renderer.sse_output_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: WIDTH as _,
                height: HEIGHT as _,
                depth_or_array_layers: 1,
            },
        );
    }

    if rendering_options.debug_pass_enabled {
        debug_pass.render(queue, &mut encoder, shader_constants, sse_texture);
    }

    scaler.render(queue, &mut encoder, shader_constants, &texture_view);

    renderer_state.queue.submit(vec![encoder.finish()]);

    renderer_state.queue.on_submitted_work_done(move || {
        log::trace!("raytracing-tt={:?}", tt.elapsed());
    });

    current_texture.present();
}

fn ease_in_ease_out(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
    }
}
