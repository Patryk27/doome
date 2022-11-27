use std::sync::atomic::AtomicUsize;

use doome_shader_common::{Projection, ShaderConstants};
use doome_wgpu_ext::AllocatedUniform;
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

pub struct DebugPass {
    pub render_pipeline: wgpu::RenderPipeline,
    projection: AllocatedUniform<Projection>,
    pub vertex_buffer: wgpu::Buffer,
    num_lines: AtomicUsize,
}

impl DebugPass {
    pub fn new(
        device: &wgpu::Device,
        shader_constants: &AllocatedUniform<ShaderConstants>,
    ) -> Self {
        let shader = device
            .create_shader_module(wgpu::include_spirv!("./debug-pass.spv"));

        let projection = AllocatedUniform::create(device, "debug_projection");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("debug_pipeline_layout"),
                bind_group_layouts: &[
                    shader_constants.bind_group_layout(),
                    projection.bind_group_layout(),
                ],
                push_constant_ranges: &[],
            });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 3]>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };

        let vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("debug_vertex_buffer"),
                contents: bytemuck::cast_slice(&[[-1.0, -1.0, 0.0]; 4056]),
                usage: wgpu::BufferUsages::VERTEX
                    | wgpu::BufferUsages::COPY_DST,
            });

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("debug_render_pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "main_vs",
                    buffers: &[vertex_buffer_layout],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "main_fs",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineList,
                    ..wgpu::PrimitiveState::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Self {
            render_pipeline,
            projection,
            vertex_buffer,
            num_lines: AtomicUsize::new(0),
        }
    }

    pub fn update_projection(&self, queue: &wgpu::Queue, view_proj: Mat4) {
        self.projection.write0(queue, &Projection::new(view_proj));
    }

    pub fn update_data(
        &self,
        queue: &wgpu::Queue,
        data: &[Vec3],
        num_lines: usize,
    ) {
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&data));
        self.num_lines
            .store(num_lines, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn render(
        &self,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        shader_contants: &AllocatedUniform<ShaderConstants>,
        output_texture: &wgpu::TextureView,
    ) {
        let mut render_pass =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("debug_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output_texture,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, shader_contants.bind_group(), &[]);
        render_pass.set_bind_group(1, self.projection.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        let num_lines =
            self.num_lines.load(std::sync::atomic::Ordering::Relaxed);
        let num_vertices = num_lines * 2;

        render_pass.draw(0..num_vertices as u32, 0..1);
    }
}
