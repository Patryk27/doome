mod cmd;

use bevy::app::AppExit;

pub use self::cmd::*;
use crate::prelude::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Command>();
        app.add_event::<CommandOutput>();
        app.add_system(handle_commands);
    }
}

pub struct CommandOutput(pub String);

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
    mut deaths: EventWriter<Death>,
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
                        MothMonster::spawn(&mut commands, &assets, position)
                    }
                    Spawnable::Heart => {
                        Heart::spawn(&mut commands, &assets, position)
                    }
                };

                output.send(CommandOutput(format!(
                    "Spawned {spawnable:?}: {}",
                    EntityHandle(entity)
                )));
            }
            Command::Despawn { entity } => {
                commands.entity(entity.0).despawn();
            }
            Command::Kill { entity } => {
                deaths.send(Death(entity.0));
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
