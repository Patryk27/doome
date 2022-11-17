use bevy::log;
use bevy::prelude::*;
use bevy::window::WindowResized;
use doome_engine::pipeline::Pipeline;
use doome_engine::{HEIGHT, WIDTH};
use doome_pixels::Pixels;
use doome_raytracer as rt;
use doome_scaler::Scaler;
use doome_wgpu_ext::AllocatedUniform;
use rt::ShaderConstants;

use crate::renderer::RendererState;

pub struct DoomePlugin;

#[derive(Resource)]
pub struct DoomeRenderer {
    pub raytracer: rt::Raytracer,
    pub scaler: Scaler,
    pub pixels: Pixels,

    intermediate_output_texture_view: wgpu::TextureView,

    shader_constants: AllocatedUniform<ShaderConstants>,
}

#[derive(Resource)]
pub struct DoomeRenderInit {
    pub pipeline: Pipeline,
}

#[derive(Resource)]
pub struct DoomeRendererContext {
    pub static_geo: Box<rt::StaticGeometry>,
    pub static_geo_index: Box<rt::StaticGeometryIndex>,
    pub dynamic_geo: Box<rt::DynamicGeometry>,
    pub geo_mapping: Box<rt::TriangleMappings>,
    pub camera: rt::Camera,
    pub lights: rt::Lights,
    pub materials: rt::Materials,
}

impl Plugin for DoomePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let renderer = app.world.resource::<RendererState>();
        let renderer_init = app.world.resource::<DoomeRenderInit>();
        let windows = app.world.resource::<Windows>();

        let device = &renderer.device;
        let queue = &renderer.queue;

        let shader_constants =
            AllocatedUniform::create(device, "shader_constants");

        let window = windows.get_primary().unwrap();

        shader_constants.write0(
            queue,
            &ShaderConstants {
                width: WIDTH as f32,
                height: HEIGHT as f32,
                // TODO: Get it
                scaled_width: window.physical_width() as f32,
                scaled_height: window.physical_height() as f32,
            },
        );

        let raytracer = rt::Raytracer::new(
            device,
            &renderer.queue,
            WIDTH as _,
            HEIGHT as _,
            renderer_init.pipeline.atlas().as_raw(),
        );

        let pixels =
            Pixels::new(device, WIDTH as _, HEIGHT as _, &shader_constants);

        let intermediate_output_texture =
            device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: WIDTH as u32,
                    height: HEIGHT as u32,
                    depth_or_array_layers: 1,
                },
                label: Some("raytracer_output"),
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
            });

        let intermediate_output_texture_view =
            intermediate_output_texture.create_view(&Default::default());

        let scaler = Scaler::new(
            device,
            &intermediate_output_texture_view,
            renderer.output_texture_format,
            &shader_constants,
        );

        app.insert_resource(DoomeRenderer {
            raytracer,
            scaler,
            pixels,
            shader_constants,
            intermediate_output_texture_view,
        });

        app.add_system(render);
        app.add_system(on_resize);
    }
}

fn render(
    renderer: Res<RendererState>,
    state: ResMut<DoomeRenderer>,
    ctxt: Res<DoomeRendererContext>,
) {
    let Ok(current_texture) = renderer.surface.get_current_texture() else { return };

    let device = &renderer.device;
    let queue = &renderer.queue;

    let intermediate_texture = &state.intermediate_output_texture_view;

    let DoomeRenderer {
        raytracer,
        pixels,
        scaler,
        shader_constants,
        ..
    } = state.as_ref();

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
        &ctxt.static_geo,
        &ctxt.static_geo_index,
        &ctxt.dynamic_geo,
        &ctxt.geo_mapping,
        &ctxt.camera,
        &ctxt.lights,
        &ctxt.materials,
        intermediate_texture,
    );

    pixels.render(queue, &mut encoder, shader_constants, intermediate_texture);

    scaler.render(queue, &mut encoder, shader_constants, &texture_view);

    renderer.queue.submit(vec![encoder.finish()]);
    current_texture.present();
}

fn on_resize(
    mut window_resized: EventReader<WindowResized>,
    renderer: Res<RendererState>,
    state: ResMut<DoomeRenderer>,
) {
    for window_resized in window_resized.iter() {
        let width = window_resized.width;
        let height = window_resized.height;

        log::info!("Window resized to ({width}, {height})");

        let RendererState {
            surface,
            device,
            queue,
            output_texture_format,
            ..
        } = renderer.as_ref();
        let DoomeRenderer {
            shader_constants, ..
        } = state.as_ref();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *output_texture_format,
            width: width as u32,
            height: height as u32,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(device, &config);

        shader_constants.write0(
            queue,
            &ShaderConstants {
                width: WIDTH as f32,
                height: HEIGHT as f32,
                scaled_width: width,
                scaled_height: height,
            },
        );
    }
}
