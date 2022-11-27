#![allow(clippy::type_complexity)]

pub mod assets;
pub mod audio;
pub mod billboard;
pub mod components;
pub mod convert;
pub mod doome;
pub mod health;
pub mod model_animation;
pub mod nav;
pub mod physics;
pub mod player;
pub mod renderer;
pub mod rendering_options;
pub mod simple_animations;
pub mod text;

mod raytracer;

pub mod prelude {
    pub use glam::*;

    pub use crate::assets::*;
    pub use crate::billboard::*;
    pub use crate::components::*;
    pub use crate::health::*;
    pub use crate::physics::components::*;
    pub use crate::physics::events::*;
    pub use crate::player::*;
    pub use crate::simple_animations::*;
}
