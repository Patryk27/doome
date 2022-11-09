#![no_std]

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Context {
    pub viewport_width: f32,
    pub viewport_height: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    /// Time in seconds since start of the program
    pub time: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    pub _padding: f32,
}
