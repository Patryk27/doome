mod cmd;

use bevy::app::AppExit;
use doome_bevy::physics::PhysicsEnabled;

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
    mut game_commands: EventReader<Command>,
    mut commands: Commands,
    assets: Res<Assets>,
    // Mutable resources
    mut physics_enabled: ResMut<PhysicsEnabled>,
    mut input_lock: ResMut<InputLock>,
    // Queries
    mut transforms: Query<&mut Transform>,
    colliders: Query<&mut Collider>,
    mut healths: Query<&mut Health>,
    player: Query<(Entity, &Player)>,
    all_entities: Query<Entity>,
    // Even writers
    mut output: EventWriter<CommandOutput>,
    mut exit: EventWriter<AppExit>,
    mut deaths: EventWriter<Death>,
    mut sync_nav_data: EventWriter<RecalculateNavData>,
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
            Command::SyncNavData => {
                sync_nav_data.send(RecalculateNavData);
            }
            Command::NoClip => {
                physics_enabled.0 = !physics_enabled.0;
                if physics_enabled.0 {
                    output.send(CommandOutput("Physics enabled".to_string()));
                } else {
                    output.send(CommandOutput("Physics disabled".to_string()));
                }
            }
            Command::DumpPhysics => {
                use std::fmt::Write;

                let mut n = 0;
                let mut lines = String::new();
                for entity in all_entities.iter() {
                    let mut points = "{polygon}(".to_string();

                    let Ok(transform) = transforms.get(entity) else { continue };
                    let Ok(collider) = colliders.get(entity) else { continue };

                    let polygon = collider.to_polygon(transform);

                    let mut is_first = true;
                    for point in polygon.points() {
                        writeln!(
                            lines,
                            "a_{{{n}}} = ({}, {})",
                            point.x, point.y
                        )
                        .unwrap();

                        if is_first {
                            is_first = false;
                        } else {
                            points.push_str(", ");
                        }

                        write!(points, "a_{{{n}}}").unwrap();

                        n += 1;
                    }

                    writeln!(lines, "{points})").unwrap();
                }

                std::fs::write("physics_dump", lines).unwrap();
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
