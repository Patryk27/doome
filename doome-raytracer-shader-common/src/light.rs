use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Light {
    pub pos: Vec4,
    pub col: Vec4,
}

impl Light {
    pub fn new(pos: Vec3, col: Vec3) -> Self {
        Self {
            pos: pos.extend(0.0),
            col: col.extend(1.0),
        }
    }
}
