use bevy::prelude::*;

#[derive(Component)]
pub struct RayCast {
    pub origin: Vec2,
    pub direction: Vec2,
    pub hit: Option<RayCastHit>,
}

pub struct RayCastHit {
    pub entity: Entity,
    pub position: Vec2,
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
