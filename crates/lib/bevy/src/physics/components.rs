use bevy::prelude::*;
use doome_geo::Polygon;

use crate::convert::{graphical_to_physical, physical_to_graphical};

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
    /// Detector colliders are not solid, they only detect collisions
    pub(super) is_detector: bool,
}

impl Collider {
    pub fn circle(radius: f32, n: usize) -> Self {
        Self {
            polygon: Polygon::circle(radius, n),
            is_detector: false,
        }
    }

    pub fn rect(width: f32, height: f32) -> Self {
        Self {
            polygon: Polygon::rect(Vec2::new(width, height)),
            is_detector: false,
        }
    }

    pub fn line(start: Vec2, end: Vec2) -> Self {
        Self {
            polygon: Polygon::line(start, end),
            is_detector: false,
        }
    }

    pub fn detector(mut self) -> Self {
        self.is_detector = true;
        self
    }

    pub fn is_detector(&self) -> bool {
        self.is_detector
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
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub body_type: BodyType,
}

pub enum BodyType {
    /// The body is moved by the physics engine and will stop on collisions
    Kinematic,
    /// The body is moved by the physics engine and will not stop on collisions
    /// Useful for bullets & maybe enemies
    Ethereal,
}

impl BodyType {
    pub fn is_kinematic(&self) -> bool {
        matches!(self, BodyType::Kinematic)
    }

    pub fn is_ethereal(&self) -> bool {
        matches!(self, BodyType::Ethereal)
    }
}
