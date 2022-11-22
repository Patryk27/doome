use bevy::prelude::*;
use doome_geo::Polygon;
use glam::Vec3Swizzles;

use super::components::*;
use super::*;

pub fn resolve_raycasts(
    mut raycasters: Query<(Entity, &Transform, &mut RayCast)>,
    colliders: Query<(Entity, &Transform, &Collider)>,
) {
    for (raycaster_entity, raycaster_transform, mut raycast) in
        raycasters.iter_mut()
    {
        raycast.hit = None;

        let (raycast_origin, _) = raycast
            .transformed_origin_and_dir(&raycaster_transform.compute_matrix());

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
    let polygon = collider.to_polygon(collider_transform);
    let (origin, dir) = raycast
        .transformed_origin_and_dir(&raycaster_transform.compute_matrix());

    polygon
        .iter_edges()
        .filter_map(|edge| {
            let edge_origin = edge.0;
            let edge_dir = edge.1 - edge.0;

            doome_geo::intersect::intersect(origin, dir, edge_origin, edge_dir)
        })
        .reduce(|curr, next| {
            if curr.distance(origin) < next.distance(origin) {
                curr
            } else {
                next
            }
        })
}
