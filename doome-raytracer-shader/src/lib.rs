#![no_std]

use doome_raytracer_shader_common::World;
use spirv_std::glam::{vec2, Vec2, Vec3, Vec4, Vec4Swizzles};
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
    #[spirv(uniform, descriptor_set = 0, binding = 0)] world: &World,
    color: &mut Vec4,
) {
    let ray = world.ray(pos.xy() / world.viewport_size);

    let mut hit_color = Vec3::default();
    let mut hit_z = None;
    let mut object_idx = 0;

    while object_idx < world.objects_count {
        let object = &world.objects[object_idx as usize];

        if let Some(new_hit_z) = object.hit(ray) {
            if new_hit_z < hit_z.unwrap_or(1000.0) {
                hit_color = object.color;
                hit_z = Some(new_hit_z);
            }
        }

        object_idx += 1;
    }

    *color = hit_color.extend(1.0);
}
