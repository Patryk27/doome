#![no_std]

use doome_shader_common::vertex_shader::full_screen_triangle;
use doome_shader_common::*;
use spirv_std::glam::{vec3, vec4, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::real::Real;
use spirv_std::{spirv, Image, Sampler};

#[spirv(vertex)]
pub fn vs_main(
    #[spirv(vertex_index)] vert_idx: i32,
    #[spirv(position)] output: &mut Vec4,
) {
    *output = full_screen_triangle(vert_idx);
}

#[allow(clippy::too_many_arguments)]
#[spirv(fragment)]
pub fn fs_main(
    #[spirv(frag_coord)] pos: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)]
    static_geo: &StaticGeometry,
    #[spirv(uniform, descriptor_set = 1, binding = 0)]
    static_geo_index: &StaticGeometryIndex,
    #[spirv(uniform, descriptor_set = 1, binding = 1)]
    dynamic_geo: &DynamicGeometry,
    #[spirv(uniform, descriptor_set = 1, binding = 2)] uvs: &TriangleUvs,
    #[spirv(uniform, descriptor_set = 2, binding = 0)] camera: &Camera,
    #[spirv(uniform, descriptor_set = 2, binding = 1)] lights: &Lights,
    #[spirv(uniform, descriptor_set = 2, binding = 2)] materials: &Materials,
    #[spirv(descriptor_set = 3, binding = 0)] atlas_tex: &Image!(2D, type=f32, sampled),
    #[spirv(descriptor_set = 3, binding = 1)] atlas_sampler: &Sampler,
    color: &mut Vec4,
) {
    let world = World {
        static_geo,
        static_geo_index,
        dynamic_geo,
        uvs,
        lights,
        materials,
        atlas_tex,
        atlas_sampler,
    };

    // *color = camera.ray(pos.xy()).shade(&world).0.extend(1.0);

    let mut ray = camera.ray(pos.xy());
    let mut bounced = false;
    let mut bounced_color = vec3(0.0, 0.0, 0.0);
    let mut bounced_reflectivity = vec4(0.0, 0.0, 0.0, 0.0);

    *color = loop {
        let (rt_color, rt_reflectivity, rt_ray) = ray.shade(&world);

        if bounced {
            break (bounced_color
                + rt_color
                    * bounced_reflectivity.xyz()
                    * bounced_reflectivity.w)
                .extend(1.0);
        }

        if rt_reflectivity.w > 0.0 {
            ray = rt_ray;
            bounced = true;
            bounced_color = rt_color;
            bounced_reflectivity = rt_reflectivity;

            continue;
        } else {
            break rt_color.extend(1.0);
        }
    };
}
