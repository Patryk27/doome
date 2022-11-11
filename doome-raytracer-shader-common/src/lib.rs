#![no_std]

pub const MAX_OBJECTS: u32 = 32;

pub mod camera;
pub mod object;
pub mod world;

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
