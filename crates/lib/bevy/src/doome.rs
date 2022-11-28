use bevy::log;
use bevy::prelude::*;
use bevy::window::{WindowResized, WindowScaleFactorChanged};
use doome_debug_pass::DebugPass;
use doome_engine::{HEIGHT, WIDTH};
use doome_pixels::Pixels;
use doome_raytracer as rt;
use doome_scaler::Scaler;
use doome_screen_space_effects::ScreenSpaceEffects;
use doome_wgpu_ext::AllocatedUniform;
use rt::ShaderConstants;

use crate::assets::Assets;
use crate::components::*;
use crate::raytracer::DoomeRaytracerPlugin;
use crate::renderer::RendererState;
use crate::rendering_options::RenderingOptions;
use crate::text::TextEngine;

pub struct DoomePlugin;

#[derive(Resource)]
pub struct DoomeRenderer {
    pub raytracer: rt::Raytracer,
    pub pixels: Pixels,
    pub screen_space_effects: ScreenSpaceEffects,
    pub debug_pass: DebugPass,
    pub scaler: Scaler,

    pub width: f32,
    pub height: f32,

    pub intermediate_output_texture: wgpu::Texture,
    pub intermediate_output_texture_view: wgpu::TextureView,
    pub sse_output_texture: wgpu::Texture,
    pub sse_output_texture_view: wgpu::TextureView,
    pub shader_constants: AllocatedUniform<ShaderConstants>,
}

impl Plugin for DoomePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RenderingOptions {
            sse_enabled: false,
            debug_pass_enabled: false,
        });

        let assets = app.world.resource::<Assets>();
        let renderer = app.world.resource::<RendererState>();
        let windows = app.world.resource::<Windows>();

        let device = &renderer.device;

        let shader_constants =
            AllocatedUniform::create(device, "shader_constants");

        let (width, height) = if let Some(window) = windows.get_primary() {
            let width = window.physical_width() as f32;
            let height = window.physical_height() as f32;

            (width, height)
        } else {
            // healthy defaults
            (1280.0, 720.0)
        };

        let raytracer = rt::Raytracer::new(
            device,
            &renderer.queue,
            WIDTH as _,
            HEIGHT as _,
            assets.atlas().as_raw(),
        );

        let pixels =
            Pixels::new(device, WIDTH as _, HEIGHT as _, &shader_constants);

        let debug_pass = DebugPass::new(device, &shader_constants);

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

        let sse_output_texture =
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
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST,
            });

        let sse_output_texture_view =
            sse_output_texture.create_view(&Default::default());

        let screen_space_effects = ScreenSpaceEffects::new(
            device,
            &intermediate_output_texture_view,
            &shader_constants,
        );

        let scaler = Scaler::new(
            device,
            &sse_output_texture_view,
            renderer.output_texture_format,
            &shader_constants,
        );

        app.insert_resource(DoomeRenderer {
            raytracer,
            pixels,
            debug_pass,
            screen_space_effects,
            scaler,
            width,
            height,
            shader_constants,
            intermediate_output_texture,
            intermediate_output_texture_view,
            sse_output_texture,
            sse_output_texture_view,
        });

        // TODO
        app.insert_resource(TextEngine::new());

        app.add_system(on_resize)
            .add_system(report_scale_factor_changes)
            .add_system(LightFade::animate)
            .add_plugin(DoomeRaytracerPlugin);
    }
}

fn on_resize(
    windows: Res<bevy::window::Windows>,
    mut window_resized: EventReader<WindowResized>,
    mut renderer: ResMut<RendererState>,
    mut state: ResMut<DoomeRenderer>,
) {
    for window_resized in window_resized.iter() {
        let window = windows.get(window_resized.id).unwrap();

        let scale_factor = window.scale_factor() as f32;

        let width = window_resized.width * scale_factor;
        let height = window_resized.height * scale_factor;

        log::info!("Window resized to ({width}, {height}) with scale factor {scale_factor}");

        let surface = if let Some(surface) = renderer.surface.as_ref() {
            surface
        } else {
            let surface = unsafe {
                renderer
                    .instance
                    .create_surface(&window.raw_handle().unwrap().get_handle())
            };

            renderer.surface = Some(surface);

            renderer.surface.as_ref().unwrap()
        };

        let RendererState {
            device,
            output_texture_format,
            ..
        } = renderer.as_ref();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *output_texture_format,
            width: width as u32,
            height: height as u32,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(device, &config);
        state.width = width;
        state.height = height;
    }
}

fn report_scale_factor_changes(
    mut events: EventReader<WindowScaleFactorChanged>,
) {
    for changed in events.iter() {
        log::info!("Scale Factor Changed: {changed:#?}");
    }
}
