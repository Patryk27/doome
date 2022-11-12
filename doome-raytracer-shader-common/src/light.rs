use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Light {
    pos: Vec4,
    color: Vec4,
}

impl Light {
    #[cfg(not(target_arch = "spirv"))]
    pub fn new(pos: Vec3, color: u32) -> Self {
        Self {
            pos: pos.extend(0.0),
            color: rgb_to_srgb(color).extend(1.0),
        }
    }

    pub fn pos(&self) -> Vec3 {
        self.pos.truncate()
    }

    pub fn color(&self) -> Vec3 {
        self.color.truncate()
    }
}
