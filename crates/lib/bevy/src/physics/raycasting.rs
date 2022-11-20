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
    let polygon = collider_to_polygon(collider_transform, collider);
    let (origin, dir) = raycast
        .transformed_origin_and_dir(&raycaster_transform.compute_matrix());

    polygon
        .iter_edges()
        .filter_map(|edge| raycast_edge(origin, dir, edge))
        .reduce(|curr, next| {
            if curr.distance(origin) < next.distance(origin) {
                curr
            } else {
                next
            }
        })
}

// https://stackoverflow.com/a/565282
fn raycast_edge(origin: Vec2, dir: Vec2, edge: (Vec2, Vec2)) -> Option<Vec2> {
    fn cross(a: Vec2, b: Vec2) -> f32 {
        a.x * b.y - a.y * b.x
    }

    let p = origin;
    let r = dir;

    let q = edge.0;
    let s = edge.1 - edge.0;

    let rs = cross(r, s);

    if rs == 0.0 {
        // The 2 vectors are collinear
        // TODO: This could lead to some weird edge cases
        // but I don't think it makes sense to return anyhing here
        None
    } else {
        let qp = q - p;
        let qp_cross_r = cross(qp, r);
        let t = cross(qp, s) / rs;
        let u = cross(qp, r) / rs;

        if qp_cross_r != 0.0 {
            // The lines are parallel
            None
        } else if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
            // The vectors intersect
            Some(p + r * t)
        } else {
            // Lines intersect, but vectors do not
            None
        }
    }
}
