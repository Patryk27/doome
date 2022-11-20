use std::f32::consts::PI;

use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::components::*;
use doome_bevy::physics::{Body, BodyType, CircleCollider, Collider};
use glam::vec3;

use super::utils::*;

pub fn init(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        Player,
        Transform::from_rotation(Quat::from_rotation_x(PI)),
        Body {
            velocity: vec3(0.0, 0.0, 0.0),
            body_type: BodyType::Kinematic,
        },
        Collider::Circle(CircleCollider { radius: 0.5 }),
    ));

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    lvl.floor(-5, -5, 5, 5);
    lvl.wall(-5, 5, 5, 5, 0);
    lvl.wall(5, 5, 5, -5, 1);

    lvl.model("elephant")
        .with_translation(vec3(1.0, 1.0, 2.5))
        .with_scale(Vec3::splat(0.35))
        .with_rotation(Quat::from_rotation_y(PI / 2.0))
        .with_material(
            Material::default()
                .with_color(Color::hex(0x789fbb))
                .with_reflectivity(0.5)
                .with_reflection_color(Color::hex(0xffffff)),
        )
        .spawn();

    lvl.point_light(-1.5, 3.0, 4.5, 0xffffff);
    lvl.point_light(1.5, 3.0, 2.5, 0xffffff);
}
