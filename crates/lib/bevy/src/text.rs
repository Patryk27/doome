use std::ops::Deref;

use bevy::prelude::Resource;
use doome_text as dt;

#[derive(Default, Resource)]
pub struct TextEngine {
    inner: dt::TextEngine,
}

impl TextEngine {
    // TODO
    pub(crate) fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl Deref for TextEngine {
    type Target = dt::TextEngine;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
