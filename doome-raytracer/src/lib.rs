use std::slice;

use doome_raytracer_shader_common as sc;
use uniforms::BufferItems;

mod uniforms;

pub struct Raytracer {
    width: u32,
    height: u32,
    pipeline: wgpu::RenderPipeline,
    output_texture: wgpu::Texture,

    // uniforms
    camera: BufferItems,
    world: BufferItems,
}

impl Raytracer {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let shader = wgpu::include_spirv!(env!("doome_raytracer_shader.spv"));
        let module = device.create_shader_module(shader);

        let camera =
            uniforms::prepare_buffer::<sc::Camera>(device, 0, "camera");
        let world = uniforms::prepare_buffer::<sc::World>(device, 0, "world");

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Raytracer pipeline layout"),
                bind_group_layouts: &[
                    &camera.bind_group_layout,
                    &world.bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("raytracer_pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });

        let output_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
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

        Self {
            width,
            height,
            pipeline,
            output_texture,

            world,
            camera,
        }
    }

    pub fn render(
        &self,
        world: &sc::World,
        camera: &sc::Camera,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        queue.write_buffer(
            &self.world.buffer,
            0,
            bytemuck::cast_slice(slice::from_ref(world)),
        );

        queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(slice::from_ref(camera)),
        );

        let view = self.output_texture.create_view(&Default::default());

        let mut rpass =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("raytracer_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

        rpass.set_scissor_rect(0, 0, self.width as _, self.height as _);
        rpass.set_pipeline(&self.pipeline);

        rpass.set_bind_group(0, &self.world.bind_group, &[]);
        rpass.set_bind_group(1, &self.camera.bind_group, &[]);

        rpass.draw(0..3, 0..1);
    }

    pub fn output_texture(&self) -> wgpu::TextureView {
        self.output_texture.create_view(&Default::default())
    }

    pub fn output_size(&self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        }
    }
}
