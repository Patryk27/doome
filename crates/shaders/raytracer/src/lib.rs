#![no_std]

use doome_shader_common::vertex_shader::full_screen_triangle;
use doome_shader_common::*;
use spirv_std::glam::{Vec4, Vec4Swizzles};
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

    *color = camera.ray(pos.xy()).shade(&world).extend(1.0);
}
