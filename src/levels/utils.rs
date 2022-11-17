use std::f32::consts::PI;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use doome_bevy::components::*;
use doome_bevy::physics::{Collider, LineCollider};
use glam::{vec2, vec3};

pub trait LevelBuilderExt<'w, 's> {
    fn floor(&mut self, x1: i32, z1: i32, x2: i32, z2: i32);
    fn ceiling(&mut self, x1: i32, z1: i32, x2: i32, z2: i32);
    fn wall(&mut self, x1: i32, z1: i32, x2: i32, z2: i32, rot: u8);
    fn light(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32);

    fn model<'a>(
        &'a mut self,
        name: &'static str,
        x: f32,
        y: f32,
        z: f32,
    ) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> LevelBuilderExt<'w, 's> for Commands<'w, 's> {
    fn floor(&mut self, x1: i32, z1: i32, x2: i32, z2: i32) {
        self.spawn(Floor { x1, z1, x2, z2 });
    }

    fn ceiling(&mut self, x1: i32, z1: i32, x2: i32, z2: i32) {
        self.spawn(Ceiling { x1, z1, x2, z2 });
    }

    fn wall(&mut self, x1: i32, z1: i32, x2: i32, z2: i32, rot: u8) {
        const HEIGHT: f32 = 4.0;

        let angle = (rot as f32) * (PI / 2.0);

        let center = {
            let (x1, x2) = (x1.min(x2), x1.max(x2));
            let (z1, z2) = (z1.min(z2), z1.max(z2));
            let (x1, z1) = (x1 as f32, z1 as f32);
            let (x2, z2) = (x2 as f32, z2 as f32);
            let center = vec3((x1 + x2) / 2.0, HEIGHT / 2.0, (z1 + z2) / 2.0);

            center
        };

        let collider_start = vec2(x1 as f32, z1 as f32);
        let collider_end = vec2(x2 as f32, z2 as f32);

        // TODO: Walls are weird right now, because we're giving the geometry and colliders here and absolute coordinates
        //       as opposed to coordinates in relation to the transform. As such, walls right now have the identity transform.
        let transform = Transform::IDENTITY;
        // Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, angle))
        //     .with_translation(center);

        self.spawn((
            Wall {
                x1,
                z1,
                x2,
                z2,
                rot,
            },
            Collider::Line(LineCollider {
                start: collider_start,
                end: collider_end,
            }),
            transform,
        ));
    }

    fn light(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
        self.spawn((
            Light { intensity: 1.0 },
            Transform::from_xyz(x, y, z),
            Color { r, g, b },
        ));
    }

    fn model<'a>(
        &'a mut self,
        name: &'static str,
        x: f32,
        y: f32,
        z: f32,
    ) -> EntityCommands<'w, 's, 'a> {
        self.spawn((ModelName::new(name), Transform::from_xyz(x, y, z)))
    }
}
