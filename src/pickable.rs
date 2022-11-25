use bevy::prelude::*;
use doome_bevy::prelude::{Collision, Player};

use crate::commands::Command;

pub struct PickablePlugin;

impl Plugin for PickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_pickables);
    }
}

fn handle_pickables(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    mut game_commands: EventWriter<Command>,
    pickables: Query<&Pickable>,
    player: Query<&Player>,
) {
    for collision in collisions.iter() {
        if player.get(collision.entity_a).is_ok() {
            if let Ok(pickable) = pickables.get(collision.entity_b) {
                game_commands.send(pickable.on_pickup);
                commands.entity(collision.entity_b).despawn();
            }
        }
    }
}

#[derive(Component)]
pub struct Pickable {
    pub on_pickup: Command,
}
