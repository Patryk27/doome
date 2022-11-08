#![no_std]

use spirv_std::glam::{vec2, vec4, Vec2, Vec4};
use spirv_std::spirv;

const VERTICES: [Vec2; 3] =
    [vec2(-1.0, 1.0), vec2(-1.0, -1.0), vec2(1.0, -1.0)];

#[spirv(vertex)]
pub fn vs_main(
    #[spirv(vertex_index)] in_vertex_index: i32,
    #[spirv(position, invariant)] output: &mut Vec4,
) {
    *output = VERTICES[in_vertex_index as usize].extend(0.0).extend(1.0);
}

#[spirv(fragment)]
pub fn fs_main(output: &mut Vec4) {
    *output = vec4(1.0, 1.0, 0.0, 1.0);
}
