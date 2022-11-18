use std::f32::consts::PI;

use bevy::prelude::*;
use doome_bevy::components::*;
use doome_bevy::physics::{Body, BodyType, CircleCollider, Collider};
use glam::vec3;

use super::utils::*;

pub fn init(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_rotation(Quat::from_rotation_x(PI)),
        Body {
            velocity: vec3(0.0, 0.0, 0.0),
            body_type: BodyType::Kinematic,
        },
        Collider::Circle(CircleCollider { radius: 0.5 }),
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
    commands.light(0.0, 3.0, -1.0, 1.0, 1.0, 1.0);

    commands
        .model("monke")
        .with_translation(vec3(-2.0, 1.0, 2.0))
        .with_rotation(Quat::from_rotation_z(0.3))
        .with_scale(Vec3::splat(0.5))
        .spawn();

    commands
        .model("monke")
        .with_translation(vec3(2.0, 1.0, 2.0))
        .with_rotation(Quat::from_rotation_z(-0.3))
        .with_scale(Vec3::splat(0.5))
        .spawn();

    let diamond = commands
        .model("diamond")
        .dynamic()
        .with_translation(vec3(0.0, 1.5, 2.0))
        .with_scale(Vec3::splat(0.4))
        .with_material(
            Material::default()
                .with_color(Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                })
                .without_texture(),
        )
        .spawn()
        .id();

    commands.insert_resource(Data { diamond });
}

#[derive(Resource)]
pub struct Data {
    diamond: Entity,
}

pub fn animate(
    data: Res<Data>,
    time: Res<Time>,
    mut xforms: Query<&mut Transform>,
    mut materials: Query<&mut Material>,
) {
    let t = time.elapsed_seconds();
    let mut xform = xforms.get_mut(data.diamond).unwrap();
    let mut material = materials.get_mut(data.diamond).unwrap();

    xform.translation.y = 1.5 + (t * 2.0).sin() / 2.0;

    xform.rotation =
        Quat::from_rotation_z(t.sin()) * Quat::from_rotation_y(t.cos());

    let color = material.color.as_mut().unwrap();

    color.r = t.sin().abs();
    color.g = t.cos().abs();
    color.b = (t.sin() + t.cos()).abs();
}
