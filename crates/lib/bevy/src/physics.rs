use bevy::prelude::*;
use glam::Vec3Swizzles;

use self::sat::{project_vertices_onto, resolve_axis_projections};

mod sat;

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Play with staging so that this gets executed at a higher frame rate

        app.add_system(update_physics)
            .add_system(resolve_collisions.before(update_physics));
    }
}

#[derive(Component, Debug)]
pub enum Collider {
    Rect(RectCollider),
    Line(LineCollider),
    Circle(CircleCollider),
}

#[derive(Debug)]
pub struct RectCollider {
    pub half_extents: Vec2,
}

#[derive(Debug)]
pub struct LineCollider {
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Debug)]
pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct Body {
    pub velocity: Vec3,
    pub body_type: BodyType,
}

pub enum BodyType {
    Static,
    Kinematic,
    // Rigid, // TODO: implement rigid body physics
}

fn resolve_collisions(
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
        let v = body.velocity * delta;
        let new_transform = active_entity_transform
            .with_translation(active_entity_transform.translation + v);

        let mut collision = false;
        for (
            passive_entity,
            passive_entity_collider,
            passive_entity_transform,
        ) in colliders.iter()
        {
            if active_entity == passive_entity {
                continue;
            }

            if are_colliding(
                &new_transform,
                active_entity_collider,
                passive_entity_transform,
                passive_entity_collider,
            ) {
                collision = true;
                break;
            }
        }

        // TODO: Calculate collision direction and only anneal the velocity in that direction
        if collision {
            body.velocity = Vec3::ZERO;
        }
    }
}

fn update_physics(time: Res<Time>, mut bodies: Query<(&Body, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (body, mut transform) in bodies.iter_mut() {
        transform.translation += body.velocity * delta;
    }
}

fn are_colliding(
    transform_a: &Transform,
    collider_a: &Collider,
    transform_b: &Transform,
    collider_b: &Collider,
) -> bool {
    match (collider_a, collider_b) {
        (Collider::Circle(a), Collider::Circle(b)) => {
            let a_pos = transform_a.translation.xz();
            let b_pos = transform_b.translation.xz();

            let distance = (a_pos - b_pos).length();

            distance < a.radius + b.radius
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
) -> bool {
    let polygon = collider_to_polygon(polygon_transform, polygon);
    let circle_center = circle_transform.translation.xz();

    let vertex_closest_to_circle = polygon
        .vertices
        .iter()
        .copied()
        .min_by(|a, b| {
            let a_distance = (*a - circle_center).length();
            let b_distance = (*b - circle_center).length();

            a_distance.partial_cmp(&b_distance).unwrap()
        })
        .unwrap();

    let circle_resolve_axis =
        (circle_center - vertex_closest_to_circle).normalize();

    let all_axes: Vec<_> = polygon
        .iter_separation_axes()
        .chain(std::iter::once(circle_resolve_axis))
        .collect();

    for axis in all_axes {
        let polygon_projections =
            project_vertices_onto(&polygon.vertices, axis);

        let circle_projections =
            project_circle_onto_axis(circle_center, circle.radius, axis);

        if resolve_axis_projections(
            axis,
            &polygon_projections,
            &circle_projections,
        ) {
            return false;
        }
    }

    true
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
