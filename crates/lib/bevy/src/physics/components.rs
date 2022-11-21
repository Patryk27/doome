use bevy::prelude::*;
use doome_geo::Polygon;
use glam::Vec3Swizzles;

const CIRCLE_POINTS: usize = 16;

#[derive(Component)]
pub struct RayCast {
    pub origin: Vec2,
    pub direction: Vec2,
    pub hit: Option<RayCastHit>,
}

impl RayCast {
    pub fn transformed_origin_and_dir(&self, matrix: &Mat4) -> (Vec2, Vec2) {
        let origin =
            matrix.transform_point3(self.origin.extend(0.0).xzy()).xz();

        let direction = matrix
            .transform_vector3(self.direction.extend(0.0).xzy())
            .xz();

        (origin, direction)
    }
}

pub struct RayCastHit {
    pub entity: Entity,
    pub position: Vec2,
}

/// The collider component for physics
///
/// Contains a polygon shape of the collider
///
/// also contains 2 masks:
///     - collision_mask: the mask of the collider
///     - collides_with_mask: the mask of the colliders that this collider can collide with
#[derive(Component, Debug)]
pub struct Collider {
    pub(super) polygon: Polygon,
    pub(super) collision_mask: u32,
}

impl Collider {
    pub fn circle(radius: f32) -> Self {
        Self {
            polygon: Polygon::circle(radius, CIRCLE_POINTS),
            collision_mask: u32::MAX,
        }
    }

    pub fn rect(width: f32, height: f32) -> Self {
        Self {
            polygon: Polygon::rect(Vec2::new(width, height)),
            collision_mask: u32::MAX,
        }
    }

    pub fn line(start: Vec2, end: Vec2) -> Self {
        Self {
            polygon: Polygon::new(vec![start, end]),
            collision_mask: u32::MAX,
        }
    }

    pub fn to_polygon(&self, transform: &Transform) -> Polygon {
        let matrix = transform.compute_matrix();
        self.polygon
            .clone()
            .map_points(|p| matrix.transform_point3(p.extend(0.0).xzy()).xz())
    }
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
