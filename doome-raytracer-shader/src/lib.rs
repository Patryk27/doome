#![no_std]

use doome_raytracer_shader_common::Uniforms;
use spirv_std::glam::{vec2, vec4, Vec2, Vec4};
use spirv_std::num_traits::real::Real;
use spirv_std::spirv;

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
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] uniforms: &Uniforms,
    output: &mut Vec4,
) {
    let coord = vec2(coord.x, coord.y) / vec2(320.0, 200.0);

    *output = vec4(coord.x, coord.y, uniforms.time.sin().abs(), 1.0);
}
