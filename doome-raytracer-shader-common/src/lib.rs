#![no_std]

mod camera;
mod object;
mod ray;
mod world;

use bytemuck::{Pod, Zeroable};
use glam::{vec2, vec4, Vec2, Vec3, Vec4, Vec4Swizzles};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::real::Real;

pub use self::camera::*;
pub use self::object::*;
pub use self::ray::*;
pub use self::world::*;

pub const MAX_OBJECTS: u32 = 32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_padding() {
        assert!(
            core::mem::size_of::<camera::Camera>() % 16 == 0,
            "Camera is not padded to 16 bytes, actual size is {}",
            core::mem::size_of::<camera::Camera>()
        );
    }

    #[test]
    fn world_padding() {
        assert!(
            core::mem::size_of::<world::World>() % 16 == 0,
            "World is not padded to 16 bytes, actual size is {}",
            core::mem::size_of::<world::World>()
        );
    }

    #[test]
    fn object_padding() {
        assert!(
            core::mem::size_of::<object::Object>() % 16 == 0,
            "Object is not padded to 16 bytes, actual size is {}",
            core::mem::size_of::<object::Object>()
        );
    }
}
