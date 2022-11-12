#![no_std]

mod camera;
mod geometry;
mod hit;
mod light;
mod lightning;
mod padded;
mod ray;
mod triangle;

use bytemuck::{Pod, Zeroable};
use glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::real::Real;

pub use self::camera::*;
pub use self::geometry::*;
pub use self::hit::*;
pub use self::light::*;
pub use self::lightning::*;
pub use self::ray::*;
pub use self::triangle::*;

pub const MAX_OBJECTS: u32 = 256;
pub const MAX_LIGHTS: u32 = 16;
