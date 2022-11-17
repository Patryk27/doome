use bevy::prelude::*;

use crate::components::Position;

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Play with staging so that this gets executed at a higher frame rate
        app.add_system(update_physics)
            .add_system(synchronize_physics_with_position);
    }
}

#[derive(Component)]
pub struct Collider {
    pub bounding_box: BoundingBox,
}

#[derive(Component)]
pub struct Body {
    pub position: Vec3,
    pub velocity: Vec3,
    pub body_type: BodyType,
}

impl Body {
    // pub fn
}

pub enum BodyType {
    Static,
    Kinematic,
    // Rigid, // TODO: implement rigid body physics
}

pub struct BoundingBox {
    pub a: Vec3,
    pub b: Vec3,
}

fn synchronize_physics_with_position(
    mut bodies_with_colliders: Query<(&Body, &mut Position)>,
) {
    for (body, mut position) in bodies_with_colliders.iter_mut() {
        position.x = body.position.x;
        position.y = body.position.y;
        position.z = body.position.z;
    }
}

fn update_physics(
    time: Res<Time>,
    mut bodies_with_colliders: Query<(&mut Body, &Collider)>,
    colliders: Query<&Collider>,
) {
    let delta = time.delta_seconds();

    for (mut body, collider) in bodies_with_colliders.iter_mut() {
        let v = body.velocity * delta;
        body.position += v;

        for collider in colliders.iter() {}
    }
}
