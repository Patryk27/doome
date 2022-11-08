#![no_std]

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Context {
    pub viewport_width: f32,
    pub viewport_height: f32,
}
