use bevy::prelude::*;
use doome_nav::{NavData, NavDataBuilder};

use crate::convert::{graphical_to_physical, physical_to_graphical};
use crate::nav::NavObstacle;
use crate::physics::Collider;
use crate::player::Player;

pub struct EnemiesPlugin;

#[derive(Default, Component)]
pub struct Enemy {
    path: Option<Vec<Vec2>>,
}

#[derive(Clone, Copy, Debug)]
pub struct RecalculateNavData;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_event::<RecalculateNavData>();
        app.add_system(recalculate_nav_data);
        app.add_system(update_hivemind);
        app.add_system(follow_player);
        app.add_system(follow_path);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Hivemind {
        nav_data: None,
        known_player_position: Vec2::ZERO,
    });
}

#[derive(Component)]
struct Hivemind {
    nav_data: Option<NavData>,
    known_player_position: Vec2,
}

const MAX_DISTANCE_FOR_NEW_PATH: f32 = 1.0;

fn update_hivemind(
    mut hivemind: Query<&mut Hivemind>,
    player: Query<(&Player, &Transform)>,
) {
    let mut hivemind = hivemind.single_mut();

    let (_player, player_transform) = player.single();
    let player_pos = graphical_to_physical(player_transform.translation);

    hivemind.known_player_position = player_pos;
}

fn follow_player(
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

const FOLLOW_SPEED: f32 = 3.0;
const NEXT_PATH_NODE_PICK_DISTANCE: f32 = 0.5;

fn follow_path(
    time: Res<Time>,
    hivemind: Query<&Hivemind>,
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
) {
    let delta = time.delta_seconds();

    let hivemind = hivemind.single();

    let player_pos = hivemind.known_player_position;

    for (mut enemy, mut transform) in enemies.iter_mut() {
        if let Some(path) = &mut enemy.path {
            if let Some(next) = path.first() {
                let pos = graphical_to_physical(transform.translation);

                let dir = *next - pos;
                let dir = dir.normalize_or_zero() * FOLLOW_SPEED * delta;
                let dir = physical_to_graphical(dir);
                transform.translation += dir;

                let pos = graphical_to_physical(transform.translation);

                if pos.distance(*next) < NEXT_PATH_NODE_PICK_DISTANCE {
                    path.remove(0);
                }
            } else {
                enemy.path = None;
            }
        }
    }
}

fn recalculate_nav_data(
    mut event_reader: EventReader<RecalculateNavData>,
    mut hivemind: Query<&mut Hivemind>,
    walls: Query<(&Transform, &Collider), With<NavObstacle>>,
) {
    if event_reader.iter().count() == 0 {
        return;
    }

    let mut hivemind = hivemind.single_mut();

    let build_time_start = std::time::Instant::now();
    let mut nav_data_builder = NavDataBuilder::new(0.6);

    for (transform, collider) in walls.iter() {
        let polygon = collider.to_polygon(transform);
        log::info!("Adding polygon: {:?}", polygon);
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
