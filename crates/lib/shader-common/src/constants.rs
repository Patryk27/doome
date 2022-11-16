use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ShaderConstants {
    pub width: f32,
    pub height: f32,
    pub scaled_width: f32,
    pub scaled_height: f32,
}
