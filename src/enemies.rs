use bevy::prelude::*;

pub struct EnemiesPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemiesPlugin {
    fn build(&self, _app: &mut App) {}
}
