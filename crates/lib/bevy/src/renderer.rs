use bevy::prelude::*;

#[derive(Default)]
pub struct RendererPlugin;

#[derive(Resource)]
pub struct RendererState {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter_info: wgpu::AdapterInfo,
    pub adapter: wgpu::Adapter,
    pub surface: Option<wgpu::Surface>,
    pub output_texture_format: wgpu::TextureFormat,
}

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let windows = app.world.resource_mut::<bevy::window::Windows>();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let window = windows.get_primary();

        let surface = window.map(|window| unsafe {
            instance.create_surface(&window.raw_handle().unwrap().get_handle())
        });

        let request_adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: surface.as_ref(),
            ..Default::default()
        };

        let (instance, device, queue, adapter_info, adapter) =
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

                (instance, device, queue, adapter.get_info(), adapter)
            });

        let output_texture_format = if let Some(surface) = surface.as_ref() {
            surface.get_supported_formats(&adapter)[0]
        } else {
            // A sensible default
            wgpu::TextureFormat::Bgra8UnormSrgb
        };

        app.world.insert_resource(RendererState {
            instance,
            output_texture_format,
            surface,
            device,
            queue,
            adapter_info,
            adapter,
        });
    }
}
