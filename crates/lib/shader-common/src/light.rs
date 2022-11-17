use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Light {
    pos: Vec4,
    color: Vec4,
}

impl Light {
    pub fn pos(&self) -> Vec3 {
        self.pos.truncate()
    }

    pub fn color(&self) -> Vec3 {
        self.color.truncate()
    }

    pub fn intensity(&self) -> f32 {
        self.color.w
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Light {
    pub fn new(pos: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            pos: pos.extend(0.0),
            color: color.extend(intensity),
        }
    }

    pub fn pos_mut(&mut self) -> &mut Vec4 {
        &mut self.pos
    }
}
