use bevy::prelude::*;

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

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub a: Vec3,
    pub b: Vec3,
}

impl BoundingBox {
    // TODO: Validate, GitHub Copilot generated this
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.a.x <= other.b.x
            && self.b.x >= other.a.x
            && self.a.y <= other.b.y
            && self.b.y >= other.a.y
            && self.a.z <= other.b.z
            && self.b.z >= other.a.z
    }

    pub fn offset(self, offset: Vec3) -> Self {
        Self {
            a: self.a + offset,
            b: self.b + offset,
        }
    }
}

fn synchronize_physics_with_position(
    mut bodies_with_colliders: Query<(&Body, &mut Transform)>,
) {
    for (body, mut transform) in bodies_with_colliders.iter_mut() {
        transform.translation = body.position;
    }
}

fn update_physics(
    time: Res<Time>,
    mut bodies_with_colliders: Query<(Entity, &mut Body, &Collider)>,
    colliders: Query<(Entity, &Collider, &Transform)>,
) {
    let delta = time.delta_seconds();

    for (body_entity, mut body, collider) in bodies_with_colliders.iter_mut() {
        let v = body.velocity * delta;
        let new_position = body.position + v;
        let new_bb = collider.bounding_box.offset(new_position);

        for (col_entity, collider, transform) in colliders.iter() {
            if body_entity == col_entity {
                continue;
            }

            let bb = collider.bounding_box.offset(transform.translation);

            log::info!("Checking collision between {new_bb:?} and {bb:?}");

            if new_bb.intersects(&bb) {
                log::info!(
                    "Collision detected between {:?} and {:?}",
                    body_entity,
                    col_entity
                );
                body.velocity = Vec3::ZERO;
            }
        }

        let v = body.velocity * delta;
        body.position = body.position + v;
    }
}
