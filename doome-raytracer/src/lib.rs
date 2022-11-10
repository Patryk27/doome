use std::slice;

use doome_raytracer_shader_common as sc;
use pixels::wgpu;

pub struct Raytracer {
    pipeline: wgpu::RenderPipeline,
    context_buffer: wgpu::Buffer,
    context_bind_group: wgpu::BindGroup,
    objects_buffer: wgpu::Buffer,
    objects_bind_group: wgpu::BindGroup,
}

impl Raytracer {
    pub fn new(pixels: &pixels::Pixels) -> Self {
        let device = pixels.device();
        let shader = wgpu::include_spirv!(env!("doome_raytracer_shader.spv"));
        let module = device.create_shader_module(shader);

        let context_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("context_buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            // TODO extract into a constant
            size: 1024,
            mapped_at_creation: false,
        });

        let context_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("context_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let context_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("context_bind_group"),
                layout: &context_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: context_buffer.as_entire_binding(),
                }],
            });

        let objects_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("objects_buffer"),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            // TODO extract into a constant
            size: 1 * 1024 * 1024,
            mapped_at_creation: false,
        });

        let objects_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("objects_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            read_only: false,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let objects_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("objects_bind_group"),
                layout: &objects_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 1,
                    resource: objects_buffer.as_entire_binding(),
                }],
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Raytracer pipeline layout"),
                bind_group_layouts: &[
                    &context_bind_group_layout,
                    &objects_bind_group_layout,
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
                        format: pixels.render_texture_format(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });

        Self {
            pipeline,
            context_buffer,
            context_bind_group,
            objects_buffer,
            objects_bind_group,
        }
    }

    pub fn render(
        &self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        context: &sc::Context,
        objects: &[sc::Object],
    ) {
        queue.write_buffer(
            &self.context_buffer,
            0,
            bytemuck::cast_slice(slice::from_ref(context)),
        );

        queue.write_buffer(
            &self.objects_buffer,
            0,
            bytemuck::cast_slice(objects),
        );

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("raytracer_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        pass.set_scissor_rect(
            0,
            0,
            context.screen_width as _,
            (context.screen_height * 0.8) as _,
        );

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.context_bind_group, &[]);
        pass.set_bind_group(1, &self.objects_bind_group, &[]);

        pass.draw(0..3, 0..1);
    }
}
