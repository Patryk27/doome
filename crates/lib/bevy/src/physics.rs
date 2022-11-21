use bevy::prelude::*;
use doome_geo::Polygon;
use glam::Vec3Swizzles;

use self::collision::resolve_collisions;

mod collision;
mod components;
mod raycasting;

pub use self::components::*;
use self::raycasting::resolve_raycasts;

#[derive(Default)]
pub struct PhysicsPlugin;

#[derive(StageLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct PhysicsStage;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            PhysicsStage,
            SystemStage::single_threaded(),
        );

        app.add_system_to_stage(PhysicsStage, update_physics)
            .add_system_to_stage(
                PhysicsStage,
                resolve_collisions.before(update_physics),
            )
            .add_system_to_stage(
                PhysicsStage,
                resolve_raycasts.after(update_physics),
            );
    }
}

fn update_physics(time: Res<Time>, mut bodies: Query<(&Body, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (body, mut transform) in bodies.iter_mut() {
        transform.translation += body.velocity * delta;
    }
}

fn collider_to_polygon(transform: &Transform, collider: &Collider) -> Polygon {
    let matrix = transform.compute_matrix();
    collider
        .polygon
        .clone()
        .map_points(|p| matrix.transform_point3(p.extend(0.0).xzy()).xz())
}
