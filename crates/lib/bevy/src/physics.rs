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

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Play with staging so that this gets executed at a higher frame rate

        app.add_system(update_physics)
            .add_system(resolve_collisions.before(update_physics))
            .add_system(resolve_raycasts);
    }
}

fn update_physics(time: Res<Time>, mut bodies: Query<(&Body, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (body, mut transform) in bodies.iter_mut() {
        transform.translation += body.velocity * delta;
    }
}
