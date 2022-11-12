use crate::*;

#[cfg(not(target_arch = "spirv"))]
pub fn rgb_to_srgb(rgb: u32) -> Vec3 {
    let [_, r, g, b] = rgb.to_be_bytes();
    let convert = |c: u8| (((c as f32) / 255.0 + 0.055) / 1.055).powf(2.4);

    vec3(convert(r), convert(g), convert(b))
}
