use bevy::prelude::*;
use engine::pipeline::Pipeline;
use engine::{RAYTRACER_HEIGHT, WIDTH};
use pixels::wgpu;
use raytracer as rt;

use crate::pixels_plugin::PixelsState;

pub struct DoomePlugin;

#[derive(Resource)]
pub struct DoomeRenderer {
    raytracer: rt::Engine,
}

#[derive(Resource)]
pub struct DoomeRenderInit {
    pub pipeline: Pipeline,
}

#[derive(Resource)]
pub struct DoomeRendererContext {
    pub camera: rt::Camera,
    pub static_geo: Box<rt::StaticGeometry>,
    pub static_geo_mapping: Box<rt::StaticGeometryMapping>,
    pub static_geo_index: Box<rt::StaticGeometryIndex>,
    pub dynamic_geo: Box<rt::DynamicGeometry>,
    pub dynamic_geo_mapping: Box<rt::DynamicGeometryMapping>,
    pub lights: rt::Lights,
    pub materials: rt::Materials,
}

impl Plugin for DoomePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let pixels_state = app.world.resource::<PixelsState>();
        let pixels = pixels_state.inner();
        let renderer_init = app.world.resource::<DoomeRenderInit>();

        let raytracer = rt::Engine::new(
            pixels.device(),
            pixels.queue(),
            WIDTH as _,
            RAYTRACER_HEIGHT as _,
            renderer_init.pipeline.atlas().as_raw(),
        );

        app.insert_resource(DoomeRenderer { raytracer });
        app.add_system(render);
    }
}

fn render(
    state: ResMut<DoomeRenderer>,
    mut pixels: ResMut<PixelsState>,
    ctxt: Res<DoomeRendererContext>,
    window: Res<Windows>,
) {
    let raytracer = &state.raytracer;
    let window = window.get_primary().unwrap();
    let window_size = (window.physical_width(), window.physical_height());

    pixels.render(window_size, |encoder, view, pixels_ctxt| {
        raytracer.render(
            &pixels_ctxt.queue,
            encoder,
            &ctxt.camera,
            &ctxt.static_geo,
            &ctxt.static_geo_mapping,
            &ctxt.static_geo_index,
            &ctxt.dynamic_geo,
            &ctxt.dynamic_geo_mapping,
            &ctxt.lights,
            &ctxt.materials,
        );

        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: raytracer.output_texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &pixels_ctxt.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: WIDTH as _,
                height: RAYTRACER_HEIGHT as _,
                depth_or_array_layers: 1,
            },
        );

        pixels_ctxt.scaling_renderer.render(encoder, view);
    });
}
