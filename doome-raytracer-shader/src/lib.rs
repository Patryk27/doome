#![no_std]

use doome_raytracer_shader_common::*;
use spirv_std::glam::{vec2, Vec2, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::real::Real;
use spirv_std::{spirv, Image, Sampler};

#[spirv(vertex)]
pub fn vs_main(
    #[spirv(vertex_index)] vert_idx: i32,
    #[spirv(position)] pos: &mut Vec4,
) {
    let uv = vec2(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let uv = 2.0 * uv - Vec2::ONE;

    *pos = uv.extend(0.0).extend(1.0);
}

#[spirv(fragment)]
pub fn fs_main(
    #[spirv(frag_coord)] pos: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera: &Camera,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] geometry: &Geometry,
    #[spirv(uniform, descriptor_set = 2, binding = 0)] lights: &Lights,
    #[spirv(uniform, descriptor_set = 3, binding = 0)] materials: &Materials,
    #[spirv(descriptor_set = 4, binding = 0)] texture: &Image!(2D, type=f32, sampled),
    #[spirv(descriptor_set = 4, binding = 1)] sampler: &Sampler,
    color: &mut Vec4,
) {
    let texture = Texture { texture, sampler };

    *color = camera
        .ray(pos.xy())
        .shade(geometry, lights, materials, &texture)
        .extend(1.0);
}
