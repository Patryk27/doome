use bevy::app::AppExit;
use bevy::prelude::*;
use doome_bevy::health::Health;
use doome_bevy::prelude::{Assets, Player};

use crate::InputLock;

mod cmd;

pub use self::cmd::*;

pub struct CommandOutput(pub String);

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Command>();
        app.add_event::<CommandOutput>();
        app.add_system(handle_commands);
    }
}

fn handle_commands(
    mut commands: Commands,
    assets: Res<Assets>,
    mut game_commands: EventReader<Command>,
    mut output: EventWriter<CommandOutput>,
    mut input_lock: ResMut<InputLock>,
    mut transforms: Query<&mut Transform>,
    mut healths: Query<&mut Health>,
    player: Query<(Entity, &Player)>,
    all_entities: Query<Entity>,
    mut exit: EventWriter<AppExit>,
) {
    for cmd in game_commands.iter().copied() {
        log::info!("Handling command: {cmd:?}");

        match cmd {
            Command::Quit => {
                exit.send(AppExit);
            }
            Command::LockInput => {
                input_lock.is_locked = true;
            }
            Command::UnlockInput => {
                input_lock.is_locked = false;
            }
            Command::ListEntities => {
                for entity in all_entities.iter() {
                    output.send(CommandOutput(format!("{}", entity.to_bits())));
                }
            }
            Command::Position { entity } => {
                let entity = resolve_entity(entity, &player);
                let transform = transforms.get(entity).unwrap();

                output
                    .send(CommandOutput(format!("{}", transform.translation)));
            }
            Command::Move { entity, position } => {
                let entity = resolve_entity(entity, &player);
                let mut transform = transforms.get_mut(entity).unwrap();

                transform.translation = position;
            }
            Command::SetHealth { entity, health } => {
                let entity = resolve_entity(entity, &player);

                let mut health_component = healths.get_mut(entity).unwrap();

                health_component.val = health;
            }
            Command::Heal { entity, amount } => {
                let entity = resolve_entity(entity, &player);

                let mut health_component = healths.get_mut(entity).unwrap();

                health_component.val += amount;
            }
            Command::Spawn {
                spawnable,
                position,
            } => {
                let entity = match spawnable {
                    Spawnable::MothMonster => {
                        crate::entities::spawn_moth_monster(
                            &mut commands,
                            &assets,
                            position,
                        )
                    }
                    Spawnable::Heart => crate::entities::spawn_heart(
                        &mut commands,
                        &assets,
                        position,
                    ),
                };

                output.send(CommandOutput(format!(
                    "Spawned {spawnable:?}: {}",
                    entity.to_bits()
                )));
            }
            Command::Despawn { entity } => {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn resolve_entity(
    entity: EntityOrPlayer,
    player: &Query<(Entity, &Player)>,
) -> Entity {
    match entity {
        EntityOrPlayer::Player => {
            let (player_entity, _) = player.single();
            player_entity
        }
        EntityOrPlayer::Entity(entity) => entity,
    }
}
