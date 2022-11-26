use std::f32::consts::E;

use bevy::prelude::*;
use doome_bevy::convert::{graphical_to_physical, physical_to_graphical};
use doome_bevy::nav::NavObstacle;
use doome_bevy::physics::components::{Collider, RayCast};
use doome_bevy::player::Player;
use doome_bevy::prelude::Body;
use doome_nav::{NavData, NavDataBuilder};
use instant::Instant;

use crate::weapons::Weapon;

pub struct EnemiesPlugin;

#[derive(Component)]
pub struct Enemy {
    path: Option<Vec<Vec2>>,
}

impl Enemy {
    pub fn new() -> Self {
        Self { path: None }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SyncNavData;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_event::<SyncNavData>();
        app.add_system(sync_nav_data);
        app.add_system(update_hivemind);
        app.add_system(update_shooting);
        app.add_system(assign_paths_to_enemies);
        app.add_system(enemy_movement);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Hivemind {
        nav_data: None,
        known_player_position: Vec2::ZERO,
        known_player_velocity: Vec2::ZERO,
        player_entity: None,
    });
}

#[derive(Component)]
struct Hivemind {
    nav_data: Option<NavData>,
    known_player_position: Vec2,
    known_player_velocity: Vec2,
    player_entity: Option<Entity>,
}

const MAX_DISTANCE_FOR_NEW_PATH: f32 = 1.0;

fn update_hivemind(
    mut hivemind: Query<&mut Hivemind>,
    player: Query<(Entity, &Player, &Body, &Transform)>,
) {
    let mut hivemind = hivemind.single_mut();

    let (player_entity, _player, body, player_transform) = player.single();
    let player_pos = graphical_to_physical(player_transform.translation);

    hivemind.known_player_position = player_pos;
    hivemind.known_player_velocity = body.velocity;
    hivemind.player_entity = Some(player_entity);
}

fn update_shooting(
    mut commands: Commands,
    hivemind: Query<&Hivemind>,
    mut enemies: Query<(&mut Enemy, &mut Weapon, &Transform, &RayCast)>,
) {
    let hivemind = hivemind.single();

    let Some(player_entity) = hivemind.player_entity else {
        log::warn!("Hivemind doesn't know the players entity");
        return;
    };

    let player_pos = hivemind.known_player_position;
    let player_vel = hivemind.known_player_velocity;

    for (_, mut weapon, transform, raycast) in enemies.iter_mut() {
        let pos = graphical_to_physical(transform.translation);
        let distance_to_player = (player_pos - pos).length();
        let time_to_hit = distance_to_player / weapon.definition.bullet_speed;
        let predicted_player_pos = player_pos + player_vel * time_to_hit;
        let to_predicted_pos = (predicted_player_pos - pos).normalize();

        if let Some(_) = player_entity_raycast(raycast, player_entity) {
            if weapon.can_shoot() {
                weapon.shoot(
                    &mut commands,
                    transform,
                    physical_to_graphical(to_predicted_pos),
                );
            }
        }
    }
}

fn assign_paths_to_enemies(
    hivemind: Query<&Hivemind>,
    mut enemies: Query<(&mut Enemy, &Transform)>,
) {
    let hivemind = hivemind.single();
    let Some(nav_data) = hivemind.nav_data.as_ref() else { return };

    let player_pos = hivemind.known_player_position;

    for (mut enemy, transform) in enemies.iter_mut() {
        let mut needs_new_path = false;

        if let Some(current_path) = &enemy.path {
            if current_path.is_empty() {
                enemy.path = None;
                needs_new_path = true;
            } else {
                let last_pos = current_path.last().unwrap();
                if last_pos.distance(player_pos) > MAX_DISTANCE_FOR_NEW_PATH {
                    needs_new_path = true;
                }
            }
        } else {
            needs_new_path = true;
        }

        if !needs_new_path {
            continue;
        }

        let enemy_pos = graphical_to_physical(transform.translation);
        let Some(path) = nav_data.find_path(enemy_pos, player_pos) else { continue };

        enemy.path = Some(path);
    }
}

const FOLLOW_SPEED: f32 = 4.0;
const NEXT_PATH_NODE_PICK_DISTANCE: f32 = 0.5;

fn enemy_movement(
    time: Res<Time>,
    hivemind: Query<&Hivemind>,
    mut enemies: Query<(&mut Enemy, &mut Body, &mut Transform, &RayCast)>,
) {
    let delta = time.delta_seconds();

    let hivemind = hivemind.single();

    let Some(player_entity) = hivemind.player_entity else {
        log::warn!("Hivemind doesn't know the players entity");
        return;
    };

    for (mut enemy, mut body, mut transform, raycast) in enemies.iter_mut() {
        body.velocity = Vec2::ZERO;

        if let Some(_player_pos) = player_entity_raycast(raycast, player_entity)
        {
            // TODO: Side strafing
            // do nothing
        } else {
            follow_path_to_player(&mut enemy, &mut transform, &mut body);
        }
    }
}

fn follow_path_to_player(
    enemy: &mut Enemy,
    transform: &Transform,
    body: &mut Body,
) {
    let Some(path) = &mut enemy.path else { return };

    let pos = graphical_to_physical(transform.translation);

    let mut path_items = path.drain(..).peekable();
    loop {
        let Some(next) = path_items.peek() else { return }; // return if path is empty

        if pos.distance(*next) >= NEXT_PATH_NODE_PICK_DISTANCE {
            break;
        } else {
            path_items.next().unwrap();
        }
    }

    let remaining_path_items = path_items.collect();
    *path = remaining_path_items;

    let Some(next) = path.first() else { return }; // return if path is empty

    let dir = *next - pos;
    let mut dir = dir.normalize_or_zero() * FOLLOW_SPEED;

    let pos = graphical_to_physical(transform.translation);

    body.velocity = dir;
}

fn player_entity_raycast(
    raycast: &RayCast,
    player_entity: Entity,
) -> Option<Vec2> {
    let hit = raycast.hit.as_ref()?;

    if hit.entity == player_entity {
        Some(hit.position)
    } else {
        None
    }
}

fn sync_nav_data(
    mut event_reader: EventReader<SyncNavData>,
    mut hivemind: Query<&mut Hivemind>,
    walls: Query<(&Transform, &Collider), With<NavObstacle>>,
) {
    if event_reader.iter().count() == 0 {
        return;
    }

    let mut hivemind = hivemind.single_mut();

    let build_time_start = Instant::now();
    let mut nav_data_builder = NavDataBuilder::new(0.75);

    for (transform, collider) in walls.iter() {
        let polygon = collider.to_polygon(transform);
        log::trace!("Adding polygon: {:?}", polygon);
        nav_data_builder.push_polygon(polygon);
    }

    let insert_polygons_time = build_time_start.elapsed();

    let nav_data = nav_data_builder.build();

    let build_time = build_time_start.elapsed();

    hivemind.nav_data = Some(nav_data);

    log::info!(
        "Building nav data complete - insert polygons: {:?}, build: {:?}",
        insert_polygons_time,
        build_time
    );
}
