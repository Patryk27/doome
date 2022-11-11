#![no_std]

use doome_raytracer_shader_common::Context;
use spirv_std::glam::{vec2, vec3, Vec2, Vec3, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
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
    #[spirv(frag_coord)] pos: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] context: &Context,
    output: &mut Vec4,
) {
    let pos = pos.xy() / context.viewport.size();
    let mut color = Vec3::default();
    let mut prev_hit_z = None;
    let mut object_idx = 0;

    while object_idx < context.objects_count {
        let object = &context.objects[object_idx as usize];

        if let Some(hit_z) = object.hit(pos) {
            if hit_z < prev_hit_z.unwrap_or(1000.0) {
                color = vec3(object.color_r, object.color_g, object.color_b);
                prev_hit_z = Some(hit_z);
            }
        }

        object_idx += 1;
    }

    *output = color.extend(1.0);
}
