use std::{any, mem};

pub struct AllocatedUniform {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

pub fn allocate<T>(
    device: &wgpu::Device,
    binding: u32,
    name: &str,
) -> AllocatedUniform {
    assert!(
        mem::size_of::<T>() % 16 == 0,
        "`{}` is not padded to 16 bytes - actual size is {}",
        any::type_name::<T>(),
        mem::size_of::<T>()
    );

    let size = mem::size_of::<T>();
    // pad size to 32 bytes
    let size = (size + 31) & !31;
    let size = size as _;

    log::info!(
        "Going to allocate buffer with name {name} and size {} padded to {}",
        mem::size_of::<T>(),
        size,
    );

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(&format!("{name}_buffer")),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        size,
        mapped_at_creation: false,
    });

    let bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&format!("{name}_bind_group_layout")),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(&format!("{name}_bind_group")),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        }],
    });

    AllocatedUniform {
        buffer,
        bind_group_layout,
        bind_group,
    }
}
