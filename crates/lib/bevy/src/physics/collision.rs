use bevy::prelude::*;
use glam::{vec3, Vec3Swizzles};

use super::sat::{self, project_vertices_onto, resolve_axis_projections};
use super::{Body, CircleCollider, Collider};

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
    match (collider_a, collider_b) {
        (Collider::Circle(a), Collider::Circle(b)) => {
            let a_pos = transform_a.translation.xz();
            let b_pos = transform_b.translation.xz();

            let axis = b_pos - a_pos;
            let distance = axis.length();

            if distance < a.radius + b.radius {
                let overlap = a.radius + b.radius - distance;

                Some(axis.normalize() * overlap)
            } else {
                None
            }
        }
        (polygon_a, Collider::Circle(b)) => are_polygon_and_circle_colliding(
            transform_a,
            polygon_a,
            transform_b,
            b,
        ),
        (Collider::Circle(a), polygon_b) => are_polygon_and_circle_colliding(
            transform_b,
            polygon_b,
            transform_a,
            a,
        ),
        _ => {
            let polygon_a = collider_to_polygon(transform_a, collider_a);
            let polygon_b = collider_to_polygon(transform_b, collider_b);

            sat::resolve_sat(&polygon_a, &polygon_b)
        }
    }
}

fn are_polygon_and_circle_colliding(
    polygon_transform: &Transform,
    polygon: &Collider,
    circle_transform: &Transform,
    circle: &CircleCollider,
) -> Option<Vec2> {
    let polygon = collider_to_polygon(polygon_transform, polygon);
    let circle_center = circle_transform.translation.xz();

    let vertex_closest_to_circle = polygon
        .vertices
        .iter()
        .copied()
        .reduce(|curr, x| {
            let curr_distance = (curr - circle_center).length_squared();
            let x_distance = (x - circle_center).length_squared();

            if x_distance < curr_distance {
                x
            } else {
                curr
            }
        })
        .unwrap();

    let circle_resolve_axis =
        (circle_center - vertex_closest_to_circle).normalize();

    // TODO: Code duplication with sat::resolve_sat
    let all_axes: Vec<_> = polygon
        .iter_separation_axes()
        .chain(std::iter::once(circle_resolve_axis))
        .collect();

    let mut mtvs = Vec::with_capacity(all_axes.len());
    for axis in all_axes {
        let polygon_projections =
            project_vertices_onto(&polygon.vertices, axis);

        let circle_projections =
            project_circle_onto_axis(circle_center, circle.radius, axis);

        mtvs.push(resolve_axis_projections(
            axis,
            &polygon_projections,
            &circle_projections,
        )?);
    }

    mtvs.into_iter().min_by(|a, b| {
        a.length()
            .partial_cmp(&b.length())
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

fn project_circle_onto_axis(
    center: Vec2,
    radius: f32,
    axis: Vec2,
) -> Vec<Vec2> {
    let center = axis * center.dot(axis);
    let radius_projection = radius * axis;

    vec![center + radius_projection, center - radius_projection]
}

fn collider_to_polygon(
    transform: &Transform,
    collider: &Collider,
) -> sat::Polygon {
    match collider {
        Collider::Rect(rect_collider) => {
            let half_extents = rect_collider.half_extents;

            let offsets = &[
                Vec2::new(-half_extents.x, -half_extents.y),
                Vec2::new(half_extents.x, -half_extents.y),
                Vec2::new(half_extents.x, half_extents.y),
                Vec2::new(-half_extents.x, half_extents.y),
            ];

            sat::Polygon::new(offsets_to_points(transform, offsets))
        }
        Collider::Line(line_collider) => {
            let offsets = &[line_collider.start, line_collider.end];

            sat::Polygon::new(offsets_to_points(transform, offsets))
        }
        Collider::Circle(_) => unimplemented!(),
    }
}

fn offsets_to_points(transform: &Transform, offsets: &[Vec2]) -> Vec<Vec2> {
    // We map the 3D position to the XZ plane
    let origin = transform.translation.xz();
    // We're only interested in Y-axis rotation so the order doesn't really matter
    let y_rotation = transform.rotation.to_euler(EulerRot::XYZ).1;

    // rotate offsets
    let points = offsets
        .iter()
        .map(|offset| {
            let rotated_offset = offset.rotate(Vec2::from_angle(y_rotation));

            rotated_offset + origin
        })
        .collect();

    points
}
