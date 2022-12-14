use crate::prelude::*;

pub const WINDOW_SCALE: f32 = 4.0;

#[derive(Debug, Resource)]
pub struct Settings {
    pub mouse_sensitivity: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.15,
        }
    }
}
