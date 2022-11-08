use pixels::wgpu;

pub(crate) struct Raytracer {
    texture_view: wgpu::TextureView,
    render_pipeline: wgpu::RenderPipeline,
}

impl Raytracer {
    pub(crate) fn new(
        pixels: &pixels::Pixels,
        width: u32,
        height: u32,
    ) -> Self {
        let device = pixels.device();
        let shader = wgpu::include_spirv!(env!("DOOME_RAYTRACER_SHADER"));

        let module = device.create_shader_module(shader);
        let texture_view = create_texture_view(pixels, width, height);

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Raytracer pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline =
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
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });

        Self {
            texture_view,
            render_pipeline,
        }
    }

    pub(crate) fn get_texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }

    pub(crate) fn resize(
        &mut self,
        pixels: &pixels::Pixels,
        width: u32,
        height: u32,
    ) {
        if width == 0 || height == 0 {
            return;
        }

        self.texture_view = create_texture_view(pixels, width, height);
    }

    pub(crate) fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        clip_rect: (u32, u32, u32, u32),
    ) {
        let mut rpass =
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Raytracer render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_scissor_rect(
            clip_rect.0,
            clip_rect.1,
            clip_rect.2,
            clip_rect.3,
        );
        rpass.draw(0..3, 0..1);
    }
}

fn create_texture_view(
    pixels: &pixels::Pixels,
    width: u32,
    height: u32,
) -> wgpu::TextureView {
    let device = pixels.device();

    let texture_descriptor = wgpu::TextureDescriptor {
        label: None,
        size: pixels::wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: pixels.render_texture_format(),
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
    };

    device
        .create_texture(&texture_descriptor)
        .create_view(&wgpu::TextureViewDescriptor::default())
}
