#![cfg_attr(target_arch = "spirv", no_std)]

use shader_common::vertex_shader::full_screen_triangle;
use shader_common::ShaderConstants;
use spirv_std::glam::{vec2, Vec4};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::{spirv, Image, Sampler};

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] pos: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)]
    constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0)] tex: &Image!(2D, type=f32, sampled),
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,
    output: &mut Vec4,
) {
    let frag_coord = vec2(
        pos.x / constants.scaled_width,
        pos.y / constants.scaled_height,
    );

    *output = tex.sample_by_lod(*sampler, frag_coord, 0.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_idx: i32,
    #[spirv(position, invariant)] output: &mut Vec4,
) {
    *output = full_screen_triangle(vert_idx);
}
