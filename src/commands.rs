mod cmd;

use std::sync::Arc;

use bevy::app::AppExit;
use bevy::ecs::system::SystemParam;
use doome_bevy::physics::PhysicsEnabled;
use doome_bevy::rendering_options::RenderingOptions;

pub use self::cmd::*;
use crate::inventory::Inventory;
use crate::music::SwitchTrack;
use crate::prelude::*;
use crate::ui;
use crate::weapons::{PrefabWeapons, Weapon, WeaponDefinition, WeaponSprites};

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Command>();
        app.add_event::<CommandOutput>();
        app.add_system(handle_commands);
    }
}

pub struct CommandOutput(pub String);

#[derive(SystemParam)]
pub struct EventWriters<'w, 's> {
    output_tx: EventWriter<'w, 's, CommandOutput>,
    exit_tx: EventWriter<'w, 's, AppExit>,
    death_tx: EventWriter<'w, 's, Death>,
    sync_nav_data_tx: EventWriter<'w, 's, SyncNavData>,
    goto_level_tx: EventWriter<'w, 's, GotoLevel>,
    switch_track_tx: EventWriter<'w, 's, SwitchTrack>,
}

#[derive(SystemParam)]
struct Queries<'w, 's> {
    colliders: Query<'w, 's, &'static Collider>,
    all_entities: Query<'w, 's, Entity>,
    player: Query<'w, 's, Entity, With<Player>>,
    enemies: Query<'w, 's, Entity, With<Enemy>>,
    weapons: Query<'w, 's, &'static mut Weapon>,
    transforms: Query<'w, 's, &'static mut Transform>,
    healths: Query<'w, 's, &'static mut Health>,
    inventory: Query<'w, 's, &'static mut Inventory>,
}

fn handle_commands(
    mut game_commands: EventReader<Command>,
    mut commands: Commands,
    assets: Res<Assets>,
    prefab_weapons: Res<PrefabWeapons>,
    // Mutable resources
    mut rendering_options: ResMut<RenderingOptions>,
    mut physics_enabled: ResMut<PhysicsEnabled>,
    mut input_lock: ResMut<InputLock>,
    mut weapon_sprites: ResMut<ui::gun::State>,
    mut enemy_ai_enabled: ResMut<EnemyAiEnabled>,
    // Queries
    mut queries: Queries,
    // Event writers
    mut event_writers: EventWriters,
) {
    for cmd in game_commands.iter().cloned() {
        log::info!("Handling command: {cmd:?}");

        match cmd {
            Command::Quit => {
                event_writers.exit_tx.send(AppExit);
            }

            Command::LockInput => {
                input_lock.is_locked = true;
            }

            Command::UnlockInput => {
                input_lock.is_locked = false;
            }

            Command::ListEntities => {
                for entity in queries.all_entities.iter() {
                    event_writers
                        .output_tx
                        .send(CommandOutput(format!("{}", entity.to_bits())));
                }
            }

            Command::Position { entity } => {
                let entity = resolve_entity(entity, &queries.player);
                let transform = queries.transforms.get(entity).unwrap();

                event_writers
                    .output_tx
                    .send(CommandOutput(format!("{}", transform.translation)));
            }

            Command::Move { entity, position } => {
                let entity = resolve_entity(entity, &queries.player);
                let mut transform = queries.transforms.get_mut(entity).unwrap();

                transform.translation = position;
            }

            Command::SetHealth { entity, health } => {
                let entity = resolve_entity(entity, &queries.player);
                let mut health_component =
                    queries.healths.get_mut(entity).unwrap();

                health_component.health = health;
            }

            Command::Heal { entity, amount } => {
                let entity = resolve_entity(entity, &queries.player);
                let mut health_component =
                    queries.healths.get_mut(entity).unwrap();

                health_component.heal(amount);
            }

            Command::Spawn {
                spawnable,
                position,
            } => {
                let entity = match spawnable {
                    Spawnable::MothMonster => {
                        MothMonster::spawn(&assets, &mut commands, position)
                    }
                    Spawnable::Doome => {
                        Doome::spawn(&assets, &mut commands, position)
                    }
                    Spawnable::Heart => Picker::heart()
                        .with_position(position.xz())
                        .spawn(&assets, &mut commands),
                    Spawnable::RiflePickup => Picker::rifle()
                        .with_position(position.xz())
                        .spawn(&assets, &mut commands),
                    Spawnable::RpgPickup => Picker::rpg()
                        .with_position(position.xz())
                        .spawn(&assets, &mut commands),
                };

                event_writers.output_tx.send(CommandOutput(format!(
                    "Spawned {spawnable:?}: {}",
                    EntityHandle(entity)
                )));
            }

            Command::Despawn { entity } => {
                commands.entity(entity.0).despawn();
            }

            Command::DespawnAllEnemies => {
                for entity in queries.enemies.iter() {
                    commands.entity(entity).despawn();
                }
            }

            Command::Kill { entity } => {
                let entity = resolve_entity(entity, &queries.player);

                event_writers.death_tx.send(Death(entity));
            }

            Command::SyncNavData => {
                event_writers.sync_nav_data_tx.send(SyncNavData::default());
            }

            Command::NoClip => {
                physics_enabled.0 = !physics_enabled.0;

                if physics_enabled.0 {
                    event_writers
                        .output_tx
                        .send(CommandOutput("Physics enabled".to_string()));
                } else {
                    event_writers
                        .output_tx
                        .send(CommandOutput("Physics disabled".to_string()));
                }
            }

            Command::DumpPhysics => {
                use std::fmt::Write;

                let mut n = 0;
                let mut lines = String::new();
                for entity in queries.all_entities.iter() {
                    let mut points = "{polygon}(".to_string();

                    let Ok(transform) = queries.transforms.get(entity) else { continue };
                    let Ok(collider) = queries.colliders.get(entity) else { continue };

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

            Command::GotoLevel { level } => {
                event_writers.goto_level_tx.send(GotoLevel::new(level));
            }

            Command::Give { what } => match what {
                Item::Flashlight => {
                    let mut inventory = queries.inventory.single_mut();
                    inventory.has_flashlight = true;
                }
                Item::Rifle => {
                    give_gun_to_player(
                        &queries.player,
                        &mut queries.weapons,
                        &mut weapon_sprites,
                        &prefab_weapons.rifle,
                    );
                }
                Item::RocketLauncher => {
                    give_gun_to_player(
                        &queries.player,
                        &mut queries.weapons,
                        &mut weapon_sprites,
                        &prefab_weapons.rpg,
                    );
                }
                Item::Handgun => {
                    give_gun_to_player(
                        &queries.player,
                        &mut queries.weapons,
                        &mut weapon_sprites,
                        &prefab_weapons.handgun,
                    );
                }
                Item::Key(key) => {
                    queries.inventory.single_mut().keys.push(key);
                }
            },

            Command::SwitchTrack { track } => {
                event_writers.switch_track_tx.send(SwitchTrack(track));
            }

            Command::ToggleDebug => {
                rendering_options.debug_pass_enabled =
                    !rendering_options.debug_pass_enabled;
            }

            Command::ToggleSSE => {
                rendering_options.sse_enabled = !rendering_options.sse_enabled;
            }

            Command::ToggleAi => {
                enemy_ai_enabled.0 = !enemy_ai_enabled.0;
            }
        }
    }
}

fn give_gun_to_player(
    player: &Query<Entity, With<Player>>,
    weapons: &mut Query<&mut Weapon>,
    weapon_sprites: &mut ui::gun::State,
    prefab_weapon: &(Arc<WeaponDefinition>, Arc<WeaponSprites>),
) {
    let player = player.single();
    let mut player_weapon = weapons.get_mut(player).unwrap();

    player_weapon.update_def(prefab_weapon.0.clone());
    weapon_sprites.current_weapon = prefab_weapon.1.clone();
}

fn resolve_entity(
    entity: EntityOrPlayer,
    player: &Query<Entity, With<Player>>,
) -> Entity {
    match entity {
        EntityOrPlayer::Player => player.single(),
        EntityOrPlayer::Entity(entity) => entity,
    }
}
