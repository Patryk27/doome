use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Light {
    pos: Vec4,
    color: Vec4,
}

impl Light {
    #[cfg(not(target_arch = "spirv"))]
    pub fn new(pos: Vec3) -> Self {
        Self {
            pos: pos.extend(0.0),
            color: vec4(1.0, 1.0, 1.0, 1.0),
        }
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn with_color(mut self, color: u32) -> Self {
        self.color = rgb_to_srgb(color).extend(1.0);
        self
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.color.w = intensity;
        self
    }

    pub fn pos(&self) -> Vec3 {
        self.pos.truncate()
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn pos_mut(&mut self) -> &mut Vec4 {
        &mut self.pos
    }

    pub fn color(&self) -> Vec3 {
        self.color.truncate()
    }

    pub fn intensity(&self) -> f32 {
        self.color.w
    }
}
