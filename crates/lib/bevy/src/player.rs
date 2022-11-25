use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub can_move: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { can_move: false }
    }
}
