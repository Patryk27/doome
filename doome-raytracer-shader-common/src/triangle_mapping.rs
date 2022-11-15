use crate::*;

// TODO use just two Vec4s
#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct TriangleMapping {
    pub uv0: Vec4,
    pub uv1: Vec4,
    pub uv2: Vec4,
}

impl TriangleMapping {
    pub fn new(uv0: Vec2, uv1: Vec2, uv2: Vec2) -> Self {
        Self {
            uv0: uv0.extend(0.0).extend(0.0),
            uv1: uv1.extend(0.0).extend(0.0),
            uv2: uv2.extend(0.0).extend(0.0),
        }
    }
}
