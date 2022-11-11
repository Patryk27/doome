use bytemuck::{Pod, Zeroable};
use glam::{Vec4, Vec4Swizzles};

use crate::camera::Ray;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Object {
    // x, y, z is position, 4th param is radius
    pub pos: Vec4,
    pub color: Vec4,
}

impl Object {
    pub fn hit(&self, ray: Ray) -> Option<f32> {
        let distance = ray.origin - self.pos.xyz();
        let a = ray.direction.length_squared();
        let b = (distance * 2.0).dot(ray.direction);
        let c = distance.length_squared() - self.pos.w * self.pos.w;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return None;
        }

        let disc_sq = disc * disc;
        let denom = 2.0 * a;

        let mut t = (-b - disc_sq) / denom;

        if t < f32::EPSILON {
            t = (-b + disc_sq) / denom;
        }

        if t < f32::EPSILON {
            return None;
        }

        Some(t)
    }
}
