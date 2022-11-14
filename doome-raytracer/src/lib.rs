use std::num::NonZeroU32;

use doome_raytracer_shader_common as sc;
use uniforms::AllocatedUniform;

mod uniforms;

pub const ATLAS_WIDTH: u32 = 256;
pub const ATLAS_HEIGHT: u32 = 256;

pub struct Raytracer {
    width: u32,
    height: u32,
    pipeline: wgpu::RenderPipeline,
    output_texture: wgpu::Texture,
    camera: AllocatedUniform<sc::Camera>,
    geometry: AllocatedUniform<sc::Geometry>,
    geometry_index: AllocatedUniform<sc::GeometryIndex>,
    lights: AllocatedUniform<sc::Lights>,
    materials: AllocatedUniform<sc::Materials>,

    tex_bind_group: wgpu::BindGroup,
}

impl Raytracer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
        atlas_data: &[u8],
    ) -> Self {
        let shader = wgpu::include_spirv!(env!("doome_raytracer_shader.spv"));
        let module = device.create_shader_module(shader);

        let camera = uniforms::allocate::<sc::Camera>(device, 0, "camera");

        let geometry =
            uniforms::allocate::<sc::Geometry>(device, 0, "geometry");

        let geometry_index = uniforms::allocate::<sc::GeometryIndex>(
            device,
            0,
            "geometry_index",
        );

        let lights = uniforms::allocate::<sc::Lights>(device, 0, "lights");

        let materials =
            uniforms::allocate::<sc::Materials>(device, 0, "materials");

        let tex_size = wgpu::Extent3d {
            width: ATLAS_WIDTH,
            height: ATLAS_HEIGHT,
            depth_or_array_layers: 1,
        };

        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("tex"),
            size: tex_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            atlas_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(ATLAS_HEIGHT * 4),
                rows_per_image: NonZeroU32::new(ATLAS_WIDTH),
            },
            tex_size,
        );

        let tex_view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let tex_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: None,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        let tex_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
                label: Some("tex_bind_group_layout"),
            });

        let tex_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &tex_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&tex_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&tex_sampler),
                    },
                ],
                label: Some("tex_bind_group"),
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Raytracer pipeline layout"),
                bind_group_layouts: &[
                    &camera.bind_group_layout,
                    &geometry.bind_group_layout,
                    &geometry_index.bind_group_layout,
                    &lights.bind_group_layout,
                    &materials.bind_group_layout,
                    &tex_bind_group_layout,
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
            camera,
            geometry,
            geometry_index,
            lights,
            materials,
            tex_bind_group,
        }
    }

    pub fn render(
        &self,
        camera: &sc::Camera,
        geometry: &sc::Geometry,
        geometry_index: &sc::GeometryIndex,
        lights: &sc::Lights,
        materials: &sc::Materials,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        self.camera.write(queue, camera);
        self.geometry.write(queue, geometry);
        self.geometry_index.write(queue, geometry_index);
        self.lights.write(queue, lights);
        self.materials.write(queue, materials);

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

        rpass.set_bind_group(0, &self.camera.bind_group, &[]);
        rpass.set_bind_group(1, &self.geometry.bind_group, &[]);
        rpass.set_bind_group(2, &self.geometry_index.bind_group, &[]);
        rpass.set_bind_group(3, &self.lights.bind_group, &[]);
        rpass.set_bind_group(4, &self.materials.bind_group, &[]);
        rpass.set_bind_group(5, &self.tex_bind_group, &[]);

        rpass.draw(0..3, 0..1);
    }

    pub fn output_texture_view(&self) -> wgpu::TextureView {
        self.output_texture.create_view(&Default::default())
    }
}
