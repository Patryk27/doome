use bevy::prelude::*;
use glam::Vec3Swizzles;

use super::*;

pub fn resolve_raycasts(
    mut raycasters: Query<(Entity, &Transform, &mut RayCast)>,
    colliders: Query<(Entity, &Transform, &Collider)>,
) {
    for (raycaster_entity, raycaster_transform, mut raycast) in
        raycasters.iter_mut()
    {
        raycast.hit = None;
        let raycast_origin = raycaster_transform
            .compute_matrix()
            .transform_point3(raycast.origin.extend(0.0).xzy())
            .xz();

        for (collider_entity, collider_transform, collider) in colliders.iter()
        {
            if raycaster_entity == collider_entity {
                continue;
            }

            if let Some(hit) = raycast_collider(
                raycaster_transform,
                raycast.as_ref(),
                collider_transform,
                collider,
            ) {
                if let Some(current_hit) = raycast.hit.as_mut() {
                    if raycast_origin.distance(hit)
                        < raycast_origin.distance(current_hit.position)
                    {
                        *current_hit = RayCastHit {
                            entity: collider_entity,
                            position: hit,
                        };
                    }
                } else {
                    raycast.hit = Some(RayCastHit {
                        entity: collider_entity,
                        position: hit,
                    });
                }
            }
        }
    }
}

fn raycast_collider(
    raycaster_transform: &Transform,
    raycast: &RayCast,
    collider_transform: &Transform,
    collider: &Collider,
) -> Option<Vec2> {
    match collider {
        Collider::Rect(_) => None, // TODO
        Collider::Line(_) => None, // TODO
        Collider::Circle(circle) => {
            let raycast_origin = raycaster_transform
                .compute_matrix()
                .transform_point3(raycast.origin.extend(0.0).xzy())
                .xz();
            let raycast_direction = raycaster_transform
                .compute_matrix()
                .transform_vector3(raycast.direction.extend(0.0).xzy())
                .xz();
            let raycast_end = raycast_origin + raycast_direction;

            let collider_position = collider_transform.translation.xz();
            let collider_radius = circle.radius;

            let a = raycast_direction.length_squared();
            let b = 2.0
                * (raycast_end.x - raycast_origin.x)
                * (raycast_origin.x - collider_position.x)
                + 2.0
                    * (raycast_end.y - raycast_origin.y)
                    * (raycast_origin.y - collider_position.y);
            let c = collider_position.distance_squared(raycast_origin)
                - collider_radius.powf(2.0);

            let discriminant = b.powf(2.0) - 4.0 * a * c;

            if discriminant < 0.0 {
                return None;
            }

            // TODO: Wasn't there something about solving the quadratic equation this way that made it numerically unstable?
            let discriminant = discriminant.sqrt();
            let t1 = (-b + discriminant) / (2.0 * a);
            let t2 = (-b - discriminant) / (2.0 * a);

            // We want to check the closer one first
            let r1 = f32::min(t1, t2);
            let r2 = f32::max(t1, t2);

            if r1 >= 0.0 && r1 <= raycast_direction.length() {
                Some(raycast_origin + raycast_direction * r1)
            } else if r2 >= 0.0 && r2 <= raycast_direction.length() {
                Some(raycast_origin + raycast_direction * r2)
            } else {
                None
            }
        }
    }
}
