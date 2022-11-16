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
    pub fn new(pos: Vec3) -> Self {
        Self {
            pos: pos.extend(0.0),
            color: vec4(1.0, 1.0, 1.0, 1.0),
        }
    }

    pub fn with_color(mut self, color: u32) -> Self {
        self.color = rgb_to_srgb(color).extend(1.0);
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.color.w = intensity;
        self
    }

    pub fn pos_mut(&mut self) -> &mut Vec4 {
        &mut self.pos
    }
}
