use bevy::prelude::*;
use doome_bevy::prelude::{Collision, Player};

use crate::commands::Command;
use crate::prelude::{Item, LevelGameplayEvent};

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
    mut level_tx: EventWriter<LevelGameplayEvent>,
) {
    for collision in collisions.iter() {
        if player.get(collision.entity_a).is_ok() {
            if let Ok(pickable) = pickables.get(collision.entity_b) {
                game_commands.send(pickable.on_pickup.clone());
                commands.entity(collision.entity_b).despawn();

                if let Command::Give {
                    what: Item::Key(key),
                } = &pickable.on_pickup
                {
                    level_tx.send(LevelGameplayEvent::KeyPicked(
                        key.name().to_owned(),
                    ));
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Pickable {
    pub on_pickup: Command,
}
