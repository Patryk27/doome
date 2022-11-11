#![no_std]

use bytemuck::{Pod, Zeroable};
use glam::{vec3, Vec2, Vec3};

pub const MAX_OBJECTS: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct World {
    pub camera_onb_u: Vec3,
    pub camera_onb_v: Vec3,
    pub camera_onb_w: Vec3,
    pub camera_origin: Vec3,
    pub _pad1: f32,
    pub _pad2: f32,
    pub _pad3: f32,
    pub camera_distance: f32,
    pub _pad4: f32,
    pub _pad5: f32,
    pub viewport_size: Vec2,
    pub objects: [Object; MAX_OBJECTS as _],
    pub _pad6: f32,
    pub _pad7: f32,
    pub _pad8: f32,
    pub objects_count: u32,
}

impl World {
    pub fn ray(&self, pos: Vec2) -> Ray {
        Ray {
            origin: self.camera_origin,
            direction: OrthonormalBasis::trace(
                self.camera_onb_u,
                self.camera_onb_v,
                self.camera_onb_w,
                vec3(pos.x, pos.y, -self.camera_distance),
            ),
        }
    }
}

pub struct Camera;

impl Camera {
    pub fn build(
        origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        distance: f32,
    ) -> (Vec3, Vec3, Vec3, Vec3, f32) {
        let (onb_u, onb_v, onb_w) =
            OrthonormalBasis::build(origin, look_at, up);

        (onb_u, onb_v, onb_w, origin, distance)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Object {
    pub center: Vec3,
    pub _pad1: f32,
    pub _pad2: f32,
    pub _pad3: f32,
    pub radius: f32,
    pub color: Vec3,
}

impl Object {
    pub fn hit(&self, ray: Ray) -> Option<f32> {
        let distance = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = (distance * 2.0).dot(ray.direction);
        let c = distance.length_squared() - self.radius * self.radius;
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

pub struct OrthonormalBasis;

impl OrthonormalBasis {
    pub fn build(eye: Vec3, look_at: Vec3, up: Vec3) -> (Vec3, Vec3, Vec3) {
        let w = (eye - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        (u, v, w)
    }

    pub fn trace(u: Vec3, v: Vec3, w: Vec3, vec: Vec3) -> Vec3 {
        u * vec.x + v * vec.y + w * vec.z
    }
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
