use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Camera {
    pub camera_onb_u: Vec4,
    pub camera_onb_v: Vec4,
    pub camera_onb_w: Vec4,
    pub camera_origin: Vec4,
    pub viewport_size: Vec4,
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
            camera_origin: origin.extend(camera_distance),
            viewport_size: viewport_size.extend(0.0).extend(0.0),
        }
    }
}

pub struct OrthonormalBasis;

impl OrthonormalBasis {
    pub fn build(eye: Vec3, look_at: Vec3, up: Vec3) -> (Vec4, Vec4, Vec4) {
        let w = (eye - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        (u.extend(0.0), v.extend(0.0), w.extend(0.0))
    }

    pub fn trace(u: Vec4, v: Vec4, w: Vec4, vec: Vec4) -> Vec4 {
        u * vec.x + v * vec.y + w * vec.z
    }
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
