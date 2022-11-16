mod geometry_builder;

pub mod canvas;
pub mod pipeline;

use doome_surface::Color;

pub use self::canvas::*;
pub use self::geometry_builder::*;

pub const WIDTH: u16 = 320;
pub const RAYTRACER_HEIGHT: u16 = 200;
pub const HUD_HEIGHT: u16 = 50;
pub const HEIGHT: u16 = RAYTRACER_HEIGHT + HUD_HEIGHT;
