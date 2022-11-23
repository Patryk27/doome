use bevy::app::AppExit;
use bevy::prelude::*;
use doome_bevy::health::Health;
use doome_bevy::prelude::Player;

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
    mut game_commands: EventReader<Command>,
    mut output: EventWriter<CommandOutput>,
    mut input_lock: ResMut<InputLock>,
    mut transforms: Query<&mut Transform>,
    mut healths: Query<&mut Health>,
    player: Query<(Entity, &Player)>,
    all_entities: Query<Entity>,
    mut exit: EventWriter<AppExit>,
) {
    let commands: Vec<Command> = game_commands.iter().copied().collect();

    for cmd in commands {
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
