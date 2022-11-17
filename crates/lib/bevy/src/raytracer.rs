mod dynamic_geometry_builder;
mod materials_manager;
mod static_geometry_builder;

use std::f32::consts::PI;

use bevy::prelude::*;
use doome_engine::{RAYTRACER_HEIGHT, WIDTH};
use doome_raytracer as rt;
use glam::{vec2, vec3};

use self::materials_manager::*;
use self::static_geometry_builder::*;
use crate::assets::Assets;
use crate::components::*;
use crate::doome::DoomeRenderer;
use crate::events::*;
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
                vec2(WIDTH as _, RAYTRACER_HEIGHT as _),
                PI / 2.0,
            );

            DoomeRaytracerState {
                static_geo: Default::default(),
                static_geo_index: Default::default(),
                dynamic_geo: Default::default(),
                mappings: Default::default(),
                camera,
                lights: Default::default(),
                materials: Default::default(),
            }
        };

        app.insert_resource(state)
            .add_event::<SyncStaticGeometry>()
            .add_system(sync_static_geometry)
            .add_system(sync_lights)
            .add_system(sync_camera)
            .add_system(render);

        app.world.spawn(Camera::default());
    }
}

#[derive(Resource)]
struct DoomeRaytracerState {
    static_geo: Box<rt::StaticGeometry>,
    static_geo_index: Option<Box<rt::StaticGeometryIndex>>,
    dynamic_geo: Box<rt::DynamicGeometry>,
    mappings: Box<rt::TriangleMappings>,
    camera: rt::Camera,
    lights: rt::Lights,
    materials: MaterialsManager,
}

fn sync_static_geometry(
    assets: Res<Assets>,
    mut ctxt: ResMut<DoomeRaytracerState>,
    floors: Query<&Floor>,
    ceilings: Query<&Ceiling>,
    walls: Query<&Wall>,
    models: Query<(
        &ModelName,
        &Transform,
        Option<&Color>,
        Option<&Transparent>,
        Option<&Reflective>,
    )>,
    mut rx: EventReader<SyncStaticGeometry>,
) {
    if rx.iter().count() == 0 {
        return;
    }

    let ctxt = &mut *ctxt;
    let mut static_geo = StaticGeometryBuilder::new(&mut ctxt.mappings);

    // TODO use proper material
    let dummy_mat = ctxt.materials.dummy();

    for floor in floors.iter() {
        static_geo
            .push_floor(floor.x1, floor.z1, floor.x2, floor.z2, dummy_mat);
    }

    for ceiling in ceilings.iter() {
        static_geo.push_ceiling(
            ceiling.x1, ceiling.z1, ceiling.x2, ceiling.z2, dummy_mat,
        );
    }

    for wall in walls.iter() {
        static_geo
            .push_wall(wall.x1, wall.z1, wall.x2, wall.z2, wall.rot, dummy_mat);
    }

    for (&model_name, &model_xform, model_color, model_transp, model_reflect) in
        models.iter()
    {
        let model = assets.model(model_name);

        let model_mat = ctxt
            .materials
            .allocate(model.material.materialize(model_color, model_reflect));

        let model_xform = model_xform.compute_matrix();
        let model_alpha = model_transp.map(|t| t.alpha).unwrap_or(1.0);

        static_geo.push_model(model, model_mat, model_alpha, model_xform);
    }

    ctxt.static_geo = static_geo.build();
    ctxt.static_geo_index = rt::GeometryIndexer::index(&ctxt.static_geo);
}

// TODO doing this each frame feels wonky
fn sync_lights(
    mut ctxt: ResMut<DoomeRaytracerState>,
    lights: Query<(&Light, &Transform, &Color)>,
) {
    ctxt.lights = Default::default();

    for (light, transform, color) in lights.iter() {
        let position = transform.translation;

        ctxt.lights.push(rt::Light::new(
            vec3(position.x, position.y, position.z),
            vec3(color.r, color.g, color.b),
            light.intensity,
        ));
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
    raytracer_state: Res<DoomeRaytracerState>,
) {
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

    let Some(static_geo_index) = &raytracer_state.static_geo_index else { return };

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
        &raytracer_state.static_geo,
        static_geo_index,
        &raytracer_state.dynamic_geo,
        &raytracer_state.mappings,
        &raytracer_state.camera,
        &raytracer_state.lights,
        raytracer_state.materials.library(),
        intermediate_texture,
    );

    pixels.render(queue, &mut encoder, shader_constants, intermediate_texture);
    scaler.render(queue, &mut encoder, shader_constants, &texture_view);

    renderer_state.queue.submit(vec![encoder.finish()]);
    current_texture.present();
}
