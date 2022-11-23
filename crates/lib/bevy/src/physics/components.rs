use bevy::prelude::*;
use doome_geo::Polygon;

use crate::convert::{graphical_to_physical, physical_to_graphical};

const CIRCLE_POINTS: usize = 16;

#[derive(Component)]
pub struct RayCast {
    pub origin: Vec2,
    pub direction: Vec2,
    pub hit: Option<RayCastHit>,
}

impl RayCast {
    pub fn transformed_origin_and_dir(&self, matrix: &Mat4) -> (Vec2, Vec2) {
        let origin = graphical_to_physical(
            matrix.transform_point3(physical_to_graphical(self.origin)),
        );

        let direction = graphical_to_physical(
            matrix.transform_vector3(physical_to_graphical(self.direction)),
        );

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
        self.polygon.clone().map_points(|p| {
            graphical_to_physical(
                matrix.transform_point3(physical_to_graphical(p)),
            )
        })
    }
}

#[derive(Component)]
pub struct Body {
    pub velocity: Vec2,
    pub body_type: BodyType,
}

pub enum BodyType {
    Static,
    Kinematic,
    // Rigid, // TODO: implement rigid body physics
}
