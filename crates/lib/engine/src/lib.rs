mod dynamic_geometry_builder;
mod static_geometry_builder;

pub mod canvas;
pub mod pipeline;

use doome_surface::Color;

pub use self::canvas::*;
pub use self::dynamic_geometry_builder::*;
pub use self::static_geometry_builder::*;

pub const WIDTH: u16 = 320;
pub const HEIGHT: u16 = 250;
