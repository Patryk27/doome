use bevy::prelude::*;
use doome_engine::pipeline::Pipeline;
use doome_engine::{RAYTRACER_HEIGHT, WIDTH};
use doome_raytracer as rt;
use pixels::wgpu;

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
    pub static_geometry: Box<rt::Geometry>,
    pub static_geometry_mapping: Box<rt::GeometryMapping>,
    pub static_geometry_index: rt::GeometryIndex,
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
    doome_state: ResMut<DoomeRenderer>,
    mut pixels: ResMut<PixelsState>,
    doome_ctxt: Res<DoomeRendererContext>,
    window: Res<Windows>,
) {
    let raytracer = &doome_state.raytracer;
    let window = window.get_primary().unwrap();
    let window_size = (window.physical_width(), window.physical_height());

    pixels.render(window_size, |encoder, view, pixels_ctxt| {
        raytracer.render(
            &pixels_ctxt.queue,
            encoder,
            &doome_ctxt.camera,
            &doome_ctxt.static_geometry,
            &doome_ctxt.static_geometry_mapping,
            &doome_ctxt.static_geometry_index,
            &doome_ctxt.lights,
            &doome_ctxt.materials,
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
