#![cfg_attr(target_arch = "spirv", no_std)]

use glam::{vec2, vec4, Vec2, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::{spirv, Image, Sampler};

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] pos: Vec4,
    #[spirv(descriptor_set = 0, binding = 0)] tex: &Image!(2D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    output: &mut Vec4,
) {
    let frag_coord = vec2(pos.x / 1280.0, pos.y / 720.0);

    *output = tex.sample_by_lod(*sampler, frag_coord, 0.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_idx: i32,
    #[spirv(position, invariant)] output: &mut Vec4,
) {
    let uv = vec2(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *output = pos.extend(0.0).extend(1.0);
}
