use bevy::prelude::*;
use doome_geo::sat;
use glam::{vec3, Vec3Swizzles};

use super::{Body, Collider};

const MIN_VELOCITY: f32 = 0.1;

pub fn resolve_collisions(
    time: Res<Time>,
    mut bodies_with_colliders: Query<(
        Entity,
        &mut Body,
        &Transform,
        &Collider,
    )>,
    colliders: Query<(Entity, &Collider, &Transform)>,
) {
    let delta = time.delta_seconds();

    for (
        active_entity,
        mut body,
        active_entity_transform,
        active_entity_collider,
    ) in bodies_with_colliders.iter_mut()
    {
        for (
            passive_entity,
            passive_entity_collider,
            passive_entity_transform,
        ) in colliders.iter()
        {
            if body.velocity.length() < f32::EPSILON {
                break;
            }

            if active_entity == passive_entity {
                continue;
            }

            let v = body.velocity * delta;
            let new_transform = active_entity_transform
                .with_translation(active_entity_transform.translation + v);

            if let Some(mtv) = are_colliding(
                &new_transform,
                active_entity_collider,
                passive_entity_transform,
                passive_entity_collider,
            ) {
                let mtv_component = vector_decompose(body.velocity.xz(), mtv);
                let mtv = vec3(mtv.x, 0.0, mtv.y);
                body.velocity -= mtv * mtv_component;

                if body.velocity.length() < MIN_VELOCITY {
                    body.velocity = Vec3::ZERO;
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
