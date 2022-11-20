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

    // Corridor
    lvl.floor(-1, -1, 1, 20);
    lvl.wall(1, -1, 1, 20, 1);
    lvl.wall(-1, -1, 1, -1, 2);
    lvl.wall(-1, -1, -1, 20, 3);

    lvl.spot_light(
        vec3(0.0, 4.0, 0.0),
        vec3(0.0, 0.0, 0.0),
        PI / 4.0,
        Color::hex(0xffffff),
    )
    .insert(LightFadeIn::new(0.5, 0.1));

    lvl.spot_light(
        vec3(0.0, 4.0, 6.0),
        vec3(0.0, 0.0, 6.0),
        PI / 4.0,
        Color::hex(0xffffff),
    )
    .insert(LightFadeIn::new(1.5, 0.1));

    lvl.spot_light(
        vec3(0.0, 4.0, 12.0),
        vec3(0.0, 0.0, 12.0),
        PI / 4.0,
        Color::hex(0xffffff),
    )
    .insert(LightFadeIn::new(2.5, 0.1));

    lvl.spot_light(
        vec3(0.0, 4.0, 18.0),
        vec3(0.0, 0.0, 18.0),
        PI / 4.0,
        Color::hex(0xffffff),
    )
    .insert(LightFadeIn::new(3.5, 0.1));

    // Main room
    lvl.floor(-8, 21, 8, 37);
    lvl.wall(-8, 37, 8, 37, 0);
    lvl.wall(8, 37, 8, 21, 1);
    lvl.wall(-8, 21, -2, 21, 2);
    lvl.wall(8, 21, 2, 21, 2);
    lvl.wall(-8, 37, -8, 21, 3);

    lvl.model("elephant")
        .with_translation(vec3(0.0, 1.0, (21.0 + 37.0) / 2.0))
        .with_scale(Vec3::splat(0.5))
        .with_rotation(Quat::from_rotation_y(PI / 2.0))
        .with_material(
            Material::default()
                .with_color(Color::hex(0x789fbb))
                .with_reflectivity(0.8)
                .with_reflection_color(Color::hex(0xffffff)),
        )
        .spawn();

    lvl.spot_light(
        vec3(0.0, 4.0, (21.0 + 37.0) / 2.0 - 4.0),
        vec3(0.0, 0.0, (21.0 + 37.0) / 2.0),
        PI / 2.0,
        Color::hex(0x990000),
    )
    .insert(LightFadeIn::new(4.5, 0.1));
}
