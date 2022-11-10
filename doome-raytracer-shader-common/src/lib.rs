#![no_std]

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::{vec2, Vec2};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Context {
    pub screen_width: f32,
    pub screen_height: f32,
    pub object_count: usize,
}

impl Context {
    pub fn screen_size(&self) -> Vec2 {
        vec2(self.screen_width, self.screen_height)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Object {
    pub center_x: f32,
    pub center_y: f32,
    pub center_z: f32,
    pub radius: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
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
