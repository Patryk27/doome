use bevy::prelude::{Plugin, Resource};

#[derive(Default)]
pub struct RendererPlugin;

#[derive(Resource)]
pub struct RendererState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter_info: wgpu::AdapterInfo,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface,
    pub output_texture_format: wgpu::TextureFormat,
}

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let windows = app.world.resource_mut::<bevy::window::Windows>();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let window = windows.get_primary().expect("Missing primary window");

        let surface = unsafe {
            instance.create_surface(&window.raw_handle().unwrap().get_handle())
        };

        let request_adapter_options = wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        };

        let (device, queue, adapter_info, adapter) =
            futures_lite::future::block_on(async move {
                let adapter = instance
                    .request_adapter(&request_adapter_options)
                    .await
                    .expect("Failed to find a compatible GPU adapter");

                let (device, queue) = adapter
                    .request_device(
                        &wgpu::DeviceDescriptor {
                            label: Some("device"),
                            features: wgpu::Features::default(),
                            limits: wgpu::Limits {
                                max_uniform_buffer_binding_size: 64 * 1024,
                                ..wgpu::Limits::downlevel_webgl2_defaults()
                            },
                        },
                        None,
                    )
                    .await
                    .unwrap();

                (device, queue, adapter.get_info(), adapter)
            });

        let output_texture_format = surface.get_supported_formats(&adapter)[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: output_texture_format,
            width: window.width() as u32,
            height: window.height() as u32,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        app.world.insert_resource(RendererState {
            output_texture_format,
            surface,
            device,
            queue,
            adapter_info,
            adapter,
        });
    }
}
