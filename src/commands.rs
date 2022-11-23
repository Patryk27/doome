use bevy::prelude::*;
use doome_bevy::health::Health;
use doome_bevy::prelude::Player;

use crate::InputLock;

mod cmd;

pub use self::cmd::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Command>();
        app.add_system(handle_commands);
    }
}

fn handle_commands(
    mut game_commands: EventReader<Command>,
    mut input_lock: ResMut<InputLock>,
    mut transforms: Query<(&mut Transform)>,
    mut healths: Query<(&mut Health)>,
    player: Query<(Entity, &Player)>,
) {
    let commands: Vec<Command> = game_commands.iter().copied().collect();

    for cmd in commands {
        log::info!("Handling command: {cmd:?}");

        match cmd {
            Command::Quit => {
                log::info!("Quit");
            }
            Command::LockInput => {
                input_lock.is_locked = true;
            }
            Command::UnlockInput => {
                input_lock.is_locked = false;
            }
            Command::Move { entity, position } => match entity {
                EntityOrPlayer::Entity(_) => {
                    unimplemented!()
                }
                EntityOrPlayer::Player => {
                    let (player, _) = player.single();
                    let mut transform = transforms.get_mut(player).unwrap();

                    transform.translation = position;
                }
            },
            Command::SetHealth { entity, health } => match entity {
                EntityOrPlayer::Entity(_) => {
                    unimplemented!()
                }
                EntityOrPlayer::Player => {
                    let (player, _) = player.single();
                    let mut health_component = healths.get_mut(player).unwrap();

                    health_component.val = health;
                }
            },
            // unhandled => {
            //     log::warn!("Unhandled command: {:?}", unhandled);
            // }
        }
    }
}
