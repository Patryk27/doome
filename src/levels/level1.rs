use std::f32::consts::PI;

use bevy::prelude::*;
use doome_bevy::components::*;
use doome_bevy::events::SyncStaticGeometry;
use doome_bevy::physics::{Body, BodyType, Collider, RectCollider};
use glam::{vec2, vec3};

use super::utils::*;

pub fn init(mut commands: Commands, mut tx: EventWriter<SyncStaticGeometry>) {
    commands.spawn((
        Player,
        Transform::from_rotation(Quat::from_rotation_x(PI)),
        Body {
            velocity: vec3(0.0, 0.0, 0.0),
            body_type: BodyType::Kinematic,
        },
        Collider::Rect(RectCollider {
            half_extents: vec2(0.5, 0.5),
        }),
    ));

    commands.floor(-3, -3, 3, 3);
    commands.wall(-3, 3, -1, 3, 0);
    commands.wall(1, 3, 3, 3, 0);
    commands.wall(3, 3, 3, -3, 1);
    commands.wall(-3, -3, 3, -3, 2);
    commands.wall(-3, -3, -3, 3, 3);
    commands.floor(-1, 3, 1, 5);
    commands.wall(-1, 5, 1, 5, 0);
    commands.wall(1, 3, 1, 5, 1);
    commands.wall(-1, 3, -1, 5, 3);
    commands.ceiling(-10, -10, 10, 10);
    commands.light(1.0, 2.0, -1.5, 1.0, 1.0, 1.0);

    commands.model("monke", -1.5, 1.0, 2.0);

    commands
        .model("monke", 1.5, 1.0, 2.0)
        .insert(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        })
        .insert(Reflective {
            reflectivity: 0.75,
            reflection_color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        });

    tx.send(SyncStaticGeometry);
}
