use bevy::prelude::*;
use doome_engine::pipeline::Pipeline;
use doome_engine::{ScalingTextureRenderer, HEIGHT, RAYTRACER_HEIGHT, WIDTH};
use doome_raytracer as rt;

use crate::pixels_plugin::PixelsState;

pub struct DoomePlugin;

#[derive(Resource)]
pub struct DoomeRenderer {
    raytracer: rt::Engine,
    scaler: ScalingTextureRenderer,
}

#[derive(Resource)]
pub struct DoomeRenderInit {
    pub pipeline: Pipeline,
}

#[derive(Resource)]
pub struct DoomeRendererContext {
    pub camera: rt::Camera,
    pub static_geometry: Box<rt::Geometry>,
    pub static_geometry_mapping: Box<rt::GeometryMapping>,
    pub static_geometry_index: rt::GeometryIndex,
    pub lights: rt::Lights,
    pub materials: rt::Materials,
}

impl Plugin for DoomePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let pixels_state = app.world.resource::<PixelsState>();
        let pixels = &pixels_state.pixels;

        let renderer_init = app.world.resource::<DoomeRenderInit>();

        let raytracer = rt::Engine::new(
            pixels.device(),
            pixels.queue(),
            WIDTH as _,
            RAYTRACER_HEIGHT as _,
            renderer_init.pipeline.atlas().as_raw(),
        );

        let scaler = ScalingTextureRenderer::new(
            pixels.device(),
            &raytracer.output_texture_view(),
            pixels.render_texture_format(),
            [1.0, (RAYTRACER_HEIGHT as f32) / (HEIGHT as f32)],
        );

        app.insert_resource(DoomeRenderer { raytracer, scaler });

        app.add_system(render);
    }
}

fn render(
    doome_state: Res<DoomeRenderer>,
    pixels: Res<PixelsState>,
    doome_renderer_context: Res<DoomeRendererContext>,
    window: Res<Windows>,
) {
    let raytracer = &doome_state.raytracer;
    let scaler = &doome_state.scaler;
    let window = window.get_primary().expect("Missing primary window");

    pixels
        .pixels
        .render_with(|encoder, view, context| {
            // Draw UI
            context.scaling_renderer.render(encoder, view);

            // Draw raytracer
            raytracer.render(
                &doome_renderer_context.camera,
                &doome_renderer_context.static_geometry,
                &doome_renderer_context.static_geometry_mapping,
                &doome_renderer_context.static_geometry_index,
                &doome_renderer_context.lights,
                &doome_renderer_context.materials,
                &context.queue,
                encoder,
            );
            scaler.render(
                encoder,
                view,
                window.physical_width(),
                window.physical_height(),
            );

            Ok(())
        })
        .unwrap();
}
