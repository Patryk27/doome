use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Camera {
    pub camera_onb_u: Vec3,
    pub camera_onb_v: Vec3,
    pub camera_onb_w: Vec3,
    pub camera_origin: Vec3,
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
    pub camera_distance: f32,
    pub viewport_size: Vec2,
    _pad4: f32,
    _pad5: f32,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        camera_distance: f32,
        viewport_size: Vec2,
    ) -> Self {
        let (camera_onb_u, camera_onb_v, camera_onb_w) =
            OrthonormalBasis::build(origin, look_at, up);

        Self {
            camera_onb_u,
            camera_onb_v,
            camera_onb_w,
            camera_origin: origin,
            camera_distance,
            viewport_size,
            _pad1: 0.0,
            _pad2: 0.0,
            _pad3: 0.0,
            _pad4: 0.0,
            _pad5: 0.0,
        }
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
