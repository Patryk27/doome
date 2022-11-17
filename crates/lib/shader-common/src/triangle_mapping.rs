use crate::*;

#[derive(Copy, Clone, Default)]
pub struct TriangleMapping {
    pub uv0: Vec2,
    pub uv1: Vec2,
    pub uv2: Vec2,
}

impl TriangleMapping {
    pub fn new(uv0: Vec2, uv1: Vec2, uv2: Vec2) -> Self {
        Self { uv0, uv1, uv2 }
    }
}
