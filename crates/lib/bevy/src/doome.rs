use std::sync::{Arc, Mutex};

use bevy::log;
use bevy::prelude::*;
use bevy::window::{WindowResized, WindowScaleFactorChanged};
use doome_engine::{HEIGHT, WIDTH};
use doome_pixels::Pixels;
use doome_raytracer as rt;
use doome_scaler::Scaler;
use doome_wgpu_ext::AllocatedUniform;
use rt::ShaderConstants;

use crate::assets::Assets;
use crate::components::*;
use crate::raytracer::DoomeRaytracerPlugin;
use crate::renderer::RendererState;

pub struct DoomePlugin {
    pub shader: Arc<Mutex<Option<wgpu::ShaderModuleDescriptor<'static>>>>,
}

#[derive(Resource)]
pub struct DoomeRenderer {
    pub raytracer: rt::Raytracer,
    pub scaler: Scaler,
    pub pixels: Pixels,
    pub intermediate_output_texture_view: wgpu::TextureView,
    pub shader_constants: AllocatedUniform<ShaderConstants>,
}

impl Plugin for DoomePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let assets = app.world.resource::<Assets>();
        let renderer = app.world.resource::<RendererState>();
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
            assets.atlas().as_raw(),
            self.shader.lock().unwrap().take().unwrap(),
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

        app.add_system(on_resize)
            .add_system(report_scale_factor_changes)
            .add_system(LightFade::animate)
            .add_plugin(DoomeRaytracerPlugin);
    }
}

fn on_resize(
    mut window_resized: EventReader<WindowResized>,
    renderer: Res<RendererState>,
    state: ResMut<DoomeRenderer>,
) {
    for window_resized in window_resized.iter() {
        let scale_factor = renderer.window_scale_factor;

        let width = window_resized.width * scale_factor;
        let height = window_resized.height * scale_factor;

        log::info!("Window resized to ({width}, {height}) with scale factor {scale_factor}");

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

fn report_scale_factor_changes(
    mut events: EventReader<WindowScaleFactorChanged>,
) {
    for changed in events.iter() {
        log::info!("Scale Factor Changed: {changed:#?}");
    }
}
