use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Default)]
pub struct ShaderConstants {
    pub width: f32,
    pub height: f32,
    pub scaled_width: f32,
    pub scaled_height: f32,
    pub time: f32,
    pub _pad1: f32,
    pub _pad2: f32,
    pub _pad3: f32,
}
