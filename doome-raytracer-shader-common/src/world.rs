use bytemuck::{Pod, Zeroable};
use glam::{vec3, Vec2};

use crate::camera::{Camera, OrthonormalBasis, Ray};
use crate::object::Object;
use crate::MAX_OBJECTS;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct World {
    pub objects: [Object; MAX_OBJECTS as _],
    pub objects_count: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
}

impl World {
    pub fn new(
        objects: [Object; MAX_OBJECTS as _],
        objects_count: u32,
    ) -> Self {
        Self {
            objects,
            objects_count,
            _pad1: 0,
            _pad2: 0,
            _pad3: 0,
        }
    }

    pub fn ray(&self, pos: Vec2, camera: &Camera) -> Ray {
        Ray {
            origin: camera.camera_origin,
            direction: OrthonormalBasis::trace(
                camera.camera_onb_u,
                camera.camera_onb_v,
                camera.camera_onb_w,
                vec3(pos.x, pos.y, -camera.camera_distance),
            ),
        }
    }
}
