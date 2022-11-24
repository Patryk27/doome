use bevy::prelude::*;

mod collision;
pub mod components;
pub mod events;
mod raycasting;

use self::collision::resolve_collisions;
use self::raycasting::resolve_raycasts;

#[derive(Default)]
pub struct PhysicsPlugin;

#[derive(Resource)]
pub struct PhysicsEnabled(pub bool);

#[derive(StageLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct PhysicsStage;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            PhysicsStage,
            SystemStage::single_threaded(),
        );

        app.insert_resource(PhysicsEnabled(true));

        app.add_event::<events::Collision>();

        // app.add_system_to_stage(PhysicsStage, update_physics);
        app.add_system_to_stage(PhysicsStage, resolve_collisions);
        app.add_system_to_stage(PhysicsStage, resolve_raycasts);
    }
}
