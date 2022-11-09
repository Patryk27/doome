#![no_std]

use doome_raytracer_shader_common::Uniforms;
use spirv_std::glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4};
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

fn hsv_to_rgb(rgb: Vec3) -> Vec3 {
    let h = rgb.x;
    let s = rgb.y;
    let v = rgb.z;

    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let ret = if h < 1.0 / 6.0 {
        vec3(c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        vec3(x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        vec3(0.0, c, x)
    } else if h < 4.0 / 6.0 {
        vec3(0.0, x, c)
    } else if h < 5.0 / 6.0 {
        vec3(x, 0.0, c)
    } else {
        vec3(c, 0.0, x)
    };

    ret + m
}

#[spirv(fragment)]
pub fn fs_main(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] uniforms: &Uniforms,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 1)]
    geometry: &[f32],
    output: &mut Vec4,
) {
    let coord = vec2(coord.x, coord.y)
        / vec2(uniforms.screen_width, uniforms.screen_height);

    let selected_geometry = uniforms.time.round() as usize % geometry.len();
    let selected_geometry = &geometry[selected_geometry];

    let rgb_color =
        hsv_to_rgb(vec3(coord.x * coord.y, *selected_geometry, 1.0));

    *output = vec4(rgb_color.x, rgb_color.y, rgb_color.z, 1.0);
}
