#![no_std]

mod camera;
mod geometry;
mod geometry_index;
mod hit;
mod light;
mod lights;
mod material;
mod materials;
mod ray;
mod sampling;
mod triangle;
mod utils;

use bytemuck::{Pod, Zeroable};
use glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::real::Real;

pub use self::camera::*;
pub use self::geometry::*;
pub use self::geometry_index::*;
pub use self::hit::*;
pub use self::light::*;
pub use self::lights::*;
pub use self::material::*;
pub use self::materials::*;
pub use self::ray::*;
pub use self::sampling::*;
pub use self::triangle::*;
use self::utils::*;

pub const MAX_OBJECTS: u32 = 900;
pub const MAX_LIGHTS: u32 = 16;
pub const MAX_MATERIALS: u32 = 16;
