// https://www.shadertoy.com/view/XtlSD7

#![cfg_attr(target_arch = "spirv", no_std)]

use core::f32::consts::PI;

use doome_shader_common::vertex_shader::full_screen_triangle;
use doome_shader_common::ShaderConstants;
use spirv_std::glam::{vec2, Vec2, Vec2Swizzles, Vec3, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::{spirv, Image, Sampler};

const VIGNETTE_INTENSITY: f32 = 0.3;
const SCANLINE_MIX: f32 = 0.9;
const SCANLINE_TIME_FACTOR: f32 = 0.008;
const SCANLINE_FREQ: f32 = 120.0;

const GRILLE_FREQ: f32 = 180.0;
const GRILLE_MIX: f32 = 1.0;

fn crt_curve_uv(mut uv: Vec2) -> Vec2 {
    uv = uv * 2.0 - 1.0;
    let offset = uv.yx().abs() / vec2(8.0, 4.0);
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    uv
}

fn vignette(color: &mut Vec3, uv: Vec2) {
    let mut vignette = uv.x * uv.y * (1.0 - uv.x) * (1.0 - uv.y);
    vignette = (16.0 * vignette).powf(VIGNETTE_INTENSITY).clamp(0.0, 1.0);
    *color *= vignette;
}

fn scanline(color: &mut Vec3, time: f32, uv: Vec2) {
    let scanline = SCANLINE_MIX
        + (1.0 - SCANLINE_MIX)
            * f32::cos(
                PI * (uv.y + SCANLINE_TIME_FACTOR * time) * SCANLINE_FREQ,
            );

    let scanline = scanline.clamp(0.0, 1.0);
    let grille = GRILLE_MIX
        + (1.0 - GRILLE_MIX)
            * f32::clamp(1.5 * f32::cos(PI * uv.x * GRILLE_FREQ), 0.0, 1.0);

    *color *= scanline * grille * 1.2;
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] pos: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)]
    constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0)] tex: &Image!(2D, type=f32, sampled),
    #[spirv(descriptor_set = 1, binding = 1)] sampler: &Sampler,
    output: &mut Vec4,
) {
    let frag_coord = vec2(pos.x / constants.width, pos.y / constants.height);
    // let frag_coord = crt_curve_uv(frag_coord);

    let color: Vec4 = tex.sample_by_lod(*sampler, frag_coord, 0.0);
    let mut color = color.xyz();

    // vignette(&mut color, frag_coord);
    // scanline(&mut color, constants.time, frag_coord);

    *output = color.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_idx: i32,
    #[spirv(position, invariant)] output: &mut Vec4,
) {
    *output = full_screen_triangle(vert_idx);
}
