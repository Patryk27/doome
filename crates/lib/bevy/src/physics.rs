use bevy::prelude::*;

use self::collision::resolve_collisions;

mod collision;
mod components;
mod raycasting;
mod sat;

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
