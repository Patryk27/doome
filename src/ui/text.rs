use bevy::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct Text {
    pub text: String,
    pub centered: bool,
}

impl Text {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            centered: false,
        }
    }

    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }
}
