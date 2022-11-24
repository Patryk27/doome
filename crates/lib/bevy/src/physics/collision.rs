use bevy::prelude::*;
use doome_geo::sat;

use super::components::{Body, Collider};
use super::events::Collision;
use super::PhysicsEnabled;
use crate::convert::physical_to_graphical;

pub fn resolve_collisions(
    time: Res<Time>,
    physics_enabled: Res<PhysicsEnabled>,
    mut collisions: EventWriter<Collision>,
    mut bodies_with_colliders: Query<(Entity, &mut Body, &Collider)>,
    colliders: Query<(Entity, &Collider)>,
    transforms: Query<&mut Transform>,
) {
    let delta = time.delta_seconds();

    for (active_entity, body, active_entity_collider) in
        bodies_with_colliders.iter_mut()
    {
        // SAFETY: This is safe, because we will validate that the entities are different.
        let mut active_entity_transform =
            unsafe { transforms.get_unchecked(active_entity).unwrap() };

        active_entity_transform.translation +=
            physical_to_graphical(body.velocity) * delta;

        if !physics_enabled.0 {
            continue;
        }

        for (passive_entity, passive_entity_collider) in colliders.iter() {
            if body.velocity.length() < f32::EPSILON {
                break;
            }

            if active_entity == passive_entity {
                continue;
            }

            // SAFETY: This is safe, because we validate previously that the entities are different.
            let passive_entity_transform = unsafe {
                let passive_entity_transform =
                    transforms.get_unchecked(passive_entity).unwrap();

                passive_entity_transform
            };

            if let Some(mtv) = are_colliding(
                &active_entity_transform,
                active_entity_collider,
                &passive_entity_transform,
                passive_entity_collider,
            ) {
                collisions.send(Collision {
                    entity_a: active_entity,
                    entity_b: passive_entity,
                });

                if body.body_type.is_kinematic() {
                    active_entity_transform.translation -=
                        physical_to_graphical(mtv);
                }
            }
        }
    }
}

/// Decomposes the vector c along the vectors a and a.perp, returns only the component along a
fn vector_decompose(c: Vec2, a: Vec2) -> f32 {
    c.dot(a) / a.length_squared()
}

fn are_colliding(
    transform_a: &Transform,
    collider_a: &Collider,
    transform_b: &Transform,
    collider_b: &Collider,
) -> Option<Vec2> {
    let polygon_a = collider_a.to_polygon(transform_a);
    let polygon_b = collider_b.to_polygon(transform_b);

    sat::resolve_sat(&polygon_a, &polygon_b)
}
