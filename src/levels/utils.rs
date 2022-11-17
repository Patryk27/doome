use bevy::prelude::*;
use doome_bevy::components::*;
use doome_bevy::physics::{BoundingBox, Collider};
use glam::vec3;

pub trait LevelBuilderExt {
    fn floor(&mut self, x1: i32, z1: i32, x2: i32, z2: i32);
    fn ceiling(&mut self, x1: i32, z1: i32, x2: i32, z2: i32);
    fn wall(&mut self, x1: i32, z1: i32, x2: i32, z2: i32, rot: u8);
    fn light(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32);
    fn model(&mut self, name: &'static str, x: f32, y: f32, z: f32);
}

impl LevelBuilderExt for Commands<'_, '_> {
    fn floor(&mut self, x1: i32, z1: i32, x2: i32, z2: i32) {
        self.spawn(Floor { x1, z1, x2, z2 });
    }

    fn ceiling(&mut self, x1: i32, z1: i32, x2: i32, z2: i32) {
        self.spawn(Ceiling { x1, z1, x2, z2 });
    }

    fn wall(&mut self, x1: i32, z1: i32, x2: i32, z2: i32, rot: u8) {
        self.spawn((
            Wall {
                x1,
                z1,
                x2,
                z2,
                rot,
            },
            Collider {
                bounding_box: BoundingBox {
                    a: vec3(-0.5, -0.5, -0.5),
                    b: vec3(0.5, 0.5, 0.5),
                },
            },
        ));
    }

    fn light(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
        self.spawn((
            Light { intensity: 1.0 },
            Position { x, y, z },
            Color { r, g, b },
        ));
    }

    fn model(&mut self, name: &'static str, x: f32, y: f32, z: f32) {
        self.spawn((ModelName::new(name), Position { x, y, z }));
    }
}
