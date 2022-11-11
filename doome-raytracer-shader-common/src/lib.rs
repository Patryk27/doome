#![no_std]

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::{vec2, Vec2};

pub const MAX_OBJECTS: u32 = 128;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Context {
    pub viewport: Viewport,
    pub objects: [Object; MAX_OBJECTS as _],
    pub objects_count: u32,
    pub _pad1: f32,
    pub _pad2: f32,
    pub _pad3: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Viewport {
    pub width: f32,
    pub _pad1: f32,
    pub height: f32,
    pub _pad2: f32,
}

impl Viewport {
    pub fn size(&self) -> Vec2 {
        vec2(self.width, self.height)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Object {
    pub center_x: f32,
    pub center_y: f32,
    pub center_z: f32,
    pub radius: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub _pad1: f32,
}

impl Object {
    pub fn hit(&self, ray: Vec2) -> Option<f32> {
        let is_hit = vec2(self.center_x, self.center_y).distance(ray)
            <= (self.radius * self.radius);

        if is_hit {
            Some(self.center_z)
        } else {
            None
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
#[cfg(test)]
mod tests {
    use core::{any, mem};

    use super::*;

    fn assert_aligned<T>() {
        let size_of = mem::size_of::<T>();

        if size_of % 16 != 0 {
            panic!(
                "`{}` is not 16-byte aligned (missing padding: {})",
                any::type_name::<T>(),
                size_of - (16 * (size_of / 16)),
            );
        }
    }

    #[test]
    fn alignment() {
        assert_aligned::<Context>();
        assert_aligned::<Viewport>();
        assert_aligned::<Object>();
    }
}
