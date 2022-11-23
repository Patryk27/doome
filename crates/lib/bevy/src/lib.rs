#![allow(clippy::type_complexity)]

pub mod assets;
pub mod audio;
pub mod billboard;
pub mod bullets;
pub mod components;
pub mod convert;
pub mod doome;
pub mod enemies;
pub mod health;
pub mod model_animation;
pub mod nav;
pub mod physics;
pub mod player;
pub mod renderer;
pub mod shooting;
pub mod simple_animations;
pub mod text;

mod raytracer;

pub mod prelude {
    pub use glam::*;

    pub use crate::assets::*;
    pub use crate::components::*;
    pub use crate::physics::components::*;
    pub use crate::player::*;
}
