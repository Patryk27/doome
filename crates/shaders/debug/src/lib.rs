#![cfg_attr(target_arch = "spirv", no_std)]

use doome_shader_common::{Projection, ShaderConstants};
use spirv_std::glam::{vec4, Vec3, Vec4};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main_fs(#[spirv(frag_coord)] _pos: Vec4, output: &mut Vec4) {
    *output = vec4(1.0, 0.0, 0.0, 1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(uniform, descriptor_set = 0, binding = 0)]
    _constants: &ShaderConstants,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] projection: &Projection,
    position: Vec3,
    #[spirv(position, invariant)] output: &mut Vec4,
) {
    *output = projection.view_proj() * position.extend(1.0);
}
