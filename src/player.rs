use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub can_move: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self { can_move: true }
    }
}
