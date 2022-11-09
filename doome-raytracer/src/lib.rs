use std::slice;

use bytemuck::Zeroable;
use doome_raytracer_shader_common::Uniforms;
use pixels::wgpu;
use wgpu::util::DeviceExt;

pub struct Raytracer {
    pipeline: wgpu::RenderPipeline,

    /// Uniforms
    uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,

    /// Geometry
    #[allow(unused)] // TODO: Use it
    geometry_buffer: wgpu::Buffer,
    geometry_bind_group: wgpu::BindGroup,
}

impl Raytracer {
    pub fn new(pixels: &pixels::Pixels) -> Self {
        let device = pixels.device();
        let shader = wgpu::include_spirv!(env!("doome_raytracer_shader.spv"));
        let module = device.create_shader_module(shader);

        let uniforms = Uniforms {
            time: 0.0,
            screen_width: 0.0,
            screen_height: 0.0,
            ..Uniforms::zeroed()
        };

        let uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
                label: Some("uniform_bind_group"),
            });

        let geometry_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Geometry Buffer"),
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST,
                contents: bytemuck::cast_slice::<f32, u8>(&[
                    0.0, 0.2, 0.4, 0.6, 0.8, 1.0,
                ]),
            });

        let geometry_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            read_only: true,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("geometry_bind_group_layout"),
            });

        let geometry_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &geometry_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 1,
                    resource: geometry_buffer.as_entire_binding(),
                }],
                label: Some("geometry_bind_group"),
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Raytracer pipeline layout"),
                bind_group_layouts: &[
                    &uniform_bind_group_layout,
                    &geometry_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Raytracer pipeline"),
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
            uniform_bind_group,
            uniform_buffer,
            geometry_buffer,
            geometry_bind_group,
        }
    }

    pub fn render(
        &self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        uniforms: &Uniforms,
        width: u32,
        height: u32,
    ) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(slice::from_ref(uniforms)),
        );

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Raytracer render pass"),
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

        pass.set_scissor_rect(0, 0, width, height);
        pass.set_pipeline(&self.pipeline);

        pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        pass.set_bind_group(1, &self.geometry_bind_group, &[]);

        pass.draw(0..3, 0..1);
    }
}
