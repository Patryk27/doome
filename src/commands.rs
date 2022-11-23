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
                    output.send(CommandOutput(format!("{:?}", entity)));
                }
            }
            Command::Position { entity } => match entity {
                EntityOrPlayer::Player => {
                    let (player_entity, _) = player.single();
                    let transform = transforms.get(player_entity).unwrap();
                    output.send(CommandOutput(format!(
                        "{}",
                        transform.translation
                    )));
                }
                _ => todo!(),
            },
            Command::Move { entity, position } => match entity {
                EntityOrPlayer::Player => {
                    let (player, _) = player.single();
                    let mut transform = transforms.get_mut(player).unwrap();

                    transform.translation = position;
                }
                EntityOrPlayer::Entity(_) => todo!(),
            },
            Command::SetHealth { entity, health } => match entity {
                EntityOrPlayer::Player => {
                    let (player, _) = player.single();
                    let mut health_component = healths.get_mut(player).unwrap();

                    health_component.val = health;
                }
                EntityOrPlayer::Entity(_) => todo!(),
            },
            Command::SpawnMothMonster { position } => {
                let entity = crate::entities::spawn_moth_monster(
                    &mut commands,
                    &assets,
                    position,
                );

                output.send(CommandOutput(format!(
                    "Spawned moth monster: {:?}",
                    entity
                )));
            } // unhandled => {
              //     log::warn!("Unhandled command: {:?}", unhandled);
              // }
        }
    }
}
