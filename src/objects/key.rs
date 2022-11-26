use crate::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Key {
    name: String,
    color: Color,
}

impl Key {
    pub fn new(name: impl ToString, color: Color) -> Self {
        Self {
            name: name.to_string(),
            color,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
