mod geometry_manager;
mod materials_manager;

use std::f32::consts::PI;

use bevy::prelude::*;
use doome_engine::{HEIGHT, WIDTH};
use doome_raytracer as rt;
use glam::{vec2, vec3};

use self::geometry_manager::*;
use self::materials_manager::*;
use crate::assets::Assets;
use crate::components::*;
use crate::doome::DoomeRenderer;
use crate::renderer::RendererState;

pub struct DoomeRaytracerPlugin;

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

            DoomeRaytracerState {
                geometry: Default::default(),
                camera,
                lights: Default::default(),
                materials: Default::default(),
            }
        };

        app.insert_resource(state)
            .add_system(sync_created_geometry)
            .add_system(sync_updated_geometry)
            .add_system(sync_lights)
            .add_system(sync_camera)
            .add_system(render)
            .add_system_to_stage(CoreStage::PostUpdate, sync_deleted_geometry);

        app.world.spawn(Camera::default());
    }
}

#[derive(Resource)]
struct DoomeRaytracerState {
    geometry: GeometryManager,
    camera: rt::Camera,
    lights: rt::Lights,
    materials: MaterialsManager,
}

fn sync_created_geometry(
    mut commands: Commands,
    mut ctxt: ResMut<DoomeRaytracerState>,
    assets: Res<Assets>,
    floors: Query<(Entity, &GeometryType, &Floor), Added<Floor>>,
    ceilings: Query<(Entity, &GeometryType, &Ceiling), Added<Ceiling>>,
    walls: Query<(Entity, &GeometryType, &Wall), Added<Wall>>,
    models: Query<
        (
            Entity,
            &GeometryType,
            &ModelName,
            &Transform,
            Option<&Material>,
        ),
        Added<ModelName>,
    >,
) {
    let ctxt = &mut *ctxt;
    let mut geo = ctxt.geometry.builder();

    for (entity, &geo_type, data) in floors.iter() {
        // TODO
        let mat = ctxt.materials.dummy(entity);

        geo.add_floor(
            entity, geo_type, data.x1, data.z1, data.x2, data.z2, mat,
        );
        commands.entity(entity).insert(Synced);
    }

    for (entity, &geo_type, data) in ceilings.iter() {
        // TODO
        let mat = ctxt.materials.dummy(entity);

        geo.ceiling(entity, geo_type, data.x1, data.z1, data.x2, data.z2, mat);
        commands.entity(entity).insert(Synced);
    }

    for (entity, &geo_type, data) in walls.iter() {
        // TODO
        let mat = ctxt.materials.dummy(entity);

        geo.add_wall(
            entity, geo_type, data.x1, data.z1, data.x2, data.z2, data.rot, mat,
        );
        commands.entity(entity).insert(Synced);
    }

    for (entity, &geo_type, name, &xform, mat) in models.iter() {
        let model = assets.model(name.clone());
        let xform = xform.compute_matrix();

        let mat = mat
            .cloned()
            .unwrap_or_default()
            .merge_with(model.material.materialize());

        let mat_id = ctxt.materials.alloc(entity, mat.materialize());

        geo.add_model(entity, geo_type, model, xform, mat, mat_id);
        commands.entity(entity).insert(Synced);
    }
}

fn sync_updated_geometry(
    mut ctxt: ResMut<DoomeRaytracerState>,
    assets: Res<Assets>,
    models: Query<
        (
            Entity,
            &GeometryType,
            &ModelName,
            &Transform,
            Option<&Material>,
        ),
        Or<(Changed<Transform>, Changed<Material>)>,
    >,
) {
    let ctxt = &mut *ctxt;
    let mut geo = ctxt.geometry.updater();

    for (entity, &geo_type, name, &xform, mat) in models.iter() {
        if geo_type == GeometryType::Static {
            // TODO it's fine to overwrite geometry for them
            continue;
        }

        let model = assets.model(name.clone());
        let xform = xform.compute_matrix();

        let mat = mat
            .cloned()
            .unwrap_or_default()
            .merge_with(model.material.materialize());

        let mat_id = ctxt.materials.alloc(entity, mat.materialize());

        geo.update_model(entity, model, xform, mat, mat_id);
    }
}

// TODO doing this each frame feels wonky
fn sync_lights(
    mut ctxt: ResMut<DoomeRaytracerState>,
    lights: Query<(&Light, &Transform, &Color)>,
) {
    ctxt.lights = Default::default();

    let enabled_lights = lights.iter().filter(|(light, _, _)| light.enabled);

    for (light, transform, color) in enabled_lights {
        let position = transform.translation;

        match light.kind {
            LightKind::Point => {
                ctxt.lights.push(rt::Light::point(
                    position,
                    vec3(color.r, color.g, color.b),
                    light.intensity,
                ));
            }
            LightKind::Spot { point_at, angle } => {
                ctxt.lights.push(rt::Light::spot(
                    position,
                    point_at,
                    angle,
                    vec3(color.r, color.g, color.b),
                    light.intensity,
                ));
            }
        }
    }
}

fn sync_camera(
    mut ctxt: ResMut<DoomeRaytracerState>,
    camera: Query<&Camera, Changed<Camera>>,
) {
    let Ok(camera) = camera.get_single() else { return };

    ctxt.camera.update(|origin, look_at, _| {
        *origin = camera.origin;
        *look_at = camera.look_at;
    });
}

fn render(
    renderer: Res<DoomeRenderer>,
    renderer_state: Res<RendererState>,
    mut raytracer_state: ResMut<DoomeRaytracerState>,
) {
    // let tt = instant::Instant::now();

    let Ok(current_texture) = renderer_state.surface.get_current_texture() else { return };
    let device = &renderer_state.device;
    let queue = &renderer_state.queue;
    let intermediate_texture = &renderer.intermediate_output_texture_view;

    let DoomeRenderer {
        raytracer,
        pixels,
        scaler,
        shader_constants,
        ..
    } = &*renderer;

    let raytracer_state = &mut *raytracer_state;

    let Some((
        static_geo,
        static_geo_index,
        dynamic_geo,
        mappings
    )) = raytracer_state.geometry.inner() else { return };

    let texture_view = current_texture
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_command_encoder"),
        });

    raytracer.render(
        queue,
        &mut encoder,
        static_geo,
        static_geo_index,
        dynamic_geo,
        mappings,
        &raytracer_state.camera,
        &raytracer_state.lights,
        raytracer_state.materials.inner(),
        intermediate_texture,
    );

    pixels.render(queue, &mut encoder, shader_constants, intermediate_texture);
    scaler.render(queue, &mut encoder, shader_constants, &texture_view);

    renderer_state.queue.submit(vec![encoder.finish()]);
    current_texture.present();

    // log::info!("raytracer-tt = {:?}", tt.elapsed());
}

fn sync_deleted_geometry(
    mut ctxt: ResMut<DoomeRaytracerState>,
    removed_entities: RemovedComponents<Synced>,
) {
    for entity in removed_entities.iter() {
        ctxt.geometry.free(entity);
        ctxt.materials.free(entity);
    }
}
