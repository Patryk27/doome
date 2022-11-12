#![no_std]

use doome_raytracer_shader_common::*;
use spirv_std::glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4, Vec4Swizzles};
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
    #[spirv(uniform, descriptor_set = 0, binding = 0)] camera: &Camera,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] geometry: &Geometry,
    #[spirv(uniform, descriptor_set = 2, binding = 0)] lightning: &Lightning,
    color: &mut Vec4,
) {
    fs_main_inner(pos, camera, geometry, lightning, color);
}

// Refactored into a separate method, so that rust-analyzer doesn't complain
fn fs_main_inner(
    pos: Vec4,
    camera: &Camera,
    geometry: &Geometry,
    lightning: &Lightning,
    color: &mut Vec4,
) {
    let ray = camera.ray(pos.xy());
    let mut hit = Hit::none();
    let mut hit_color = vec4(0.0, 0.0, 0.0, 1.0);

    // -----

    let tris = geometry.all();
    let mut tri_idx = 0;

    while tri_idx < geometry.len() {
        let tri = &tris[tri_idx as usize];
        let tri_hit = tri.hit(ray);

        if tri_hit.is_closer_than(hit) {
            hit = tri_hit;
            hit_color = tri.color();
        }

        tri_idx += 1;
    }

    // -----

    if hit.is_some() {
        let lights = lightning.all();
        let mat_color = hit_color.truncate();
        let mut light_idx = 0;
        let mut new_hit_color = vec3(0.0, 0.0, 0.0);

        while light_idx < lightning.len() {
            let light = lights[light_idx as usize];

            if !geometry.any_obstacle_between(hit.point, light.pos.truncate()) {
                new_hit_color += radiance(light, hit, mat_color);
            }

            light_idx += 1;
        }

        hit_color = new_hit_color.extend(1.0);
    }

    // -----

    *color = hit_color;
}

fn radiance(light: Light, hit: Hit, mat_color: Vec3) -> Vec3 {
    let direction = (light.pos.truncate() - hit.point).normalize();
    let diffuse_factor = direction.dot(hit.normal);

    if diffuse_factor < 0.0 {
        vec3(0.0, 0.0, 0.0)
    } else {
        light.col.truncate() * mat_color * diffuse_factor
    }
}
