use bevy::prelude::Resource;
use doome_text::TextEngine;

#[derive(Default, Resource)]
pub struct Text {
    pub text_engine: TextEngine,
}
