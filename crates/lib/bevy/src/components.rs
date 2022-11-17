use bevy::prelude::*;
use glam::vec3;

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    // TODO rgb to srgb?
    pub fn into_vec3(self) -> Vec3 {
        vec3(self.r, self.g, self.b)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Health {
    pub val: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Light {
    pub intensity: f32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
pub struct Camera {
    pub origin: Vec3,
    pub look_at: Vec3,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Floor {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Ceiling {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Wall {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
    pub rot: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component)]
pub struct ModelName {
    pub(crate) name: &'static str,
}

impl ModelName {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Transparent {
    pub alpha: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Reflective {
    pub reflectivity: f32,
    pub reflection_color: Color,
}
