use bevy::prelude::*;

use crate::shooting::Shooter;

#[derive(Component)]
pub struct Player {
    pub can_move: bool,
    pub shooter: Shooter,
}

impl Player {
    pub fn new(shooter: Shooter) -> Self {
        Self {
            can_move: false,
            shooter,
        }
    }
}
