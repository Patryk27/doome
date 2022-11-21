use bevy::prelude::*;

mod collision;
pub mod components;
pub mod events;
mod raycasting;

use self::collision::resolve_collisions;
use self::components::Body;
use self::raycasting::resolve_raycasts;
use crate::convert::physical_to_graphical;

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

        app.add_event::<events::Collision>();

        app.add_system_to_stage(PhysicsStage, update_physics);
        app.add_system_to_stage(
            PhysicsStage,
            resolve_collisions.before(update_physics),
        );
        app.add_system_to_stage(
            PhysicsStage,
            resolve_raycasts.after(update_physics),
        );
    }
}

fn update_physics(time: Res<Time>, mut bodies: Query<(&Body, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (body, mut transform) in bodies.iter_mut() {
        transform.translation += physical_to_graphical(body.velocity) * delta;
    }
}
