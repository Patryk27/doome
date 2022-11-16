use bevy::prelude::Resource;
use text::TextEngine;

#[derive(Default, Resource)]
pub struct Text {
    pub text_engine: TextEngine,
}
