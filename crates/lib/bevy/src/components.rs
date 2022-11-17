use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct Rotation {
    pub angle: f32,
}

#[derive(Component)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Component)]
pub struct Health {
    pub val: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Light {
    pub intensity: f32,
}

#[derive(Component, Default)]
pub struct Camera {
    pub origin: Vec3,
    pub look_at: Vec3,
}

#[derive(Component)]
pub struct Floor {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
}

#[derive(Component)]
pub struct Ceiling {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
}

#[derive(Component)]
pub struct Wall {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
    pub rot: u8,
}
