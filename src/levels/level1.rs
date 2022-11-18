use std::f32::consts::PI;
use std::time::Duration;

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

    let l1 = commands.light(-1.5, 3.0, -1.0, 0.8, 0.0, 0.0).id();
    let l2 = commands.light(0.0, 3.0, -1.0, 0.0, 0.8, 0.0).id();
    let l3 = commands.light(1.5, 3.0, -1.0, 0.0, 0.0, 0.8).id();

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

    commands
        .model("monke")
        .with_translation(vec3(0.0, 1.0, 3.0))
        .with_scale(Vec3::splat(0.5))
        .spawn();

    let d1 = commands
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

    commands.insert_resource(Data {
        l1,
        l2,
        l3,
        d1,
        d2: None,
        timer: Timer::new(Duration::from_millis(1500), TimerMode::Repeating),
    });
}

#[derive(Resource)]
pub struct Data {
    l1: Entity,
    l2: Entity,
    l3: Entity,
    d1: Entity,
    d2: Option<Entity>,
    timer: Timer,
}

pub fn animate(
    time: Res<Time>,
    mut commands: Commands,
    mut data: ResMut<Data>,
    mut xforms: Query<&mut Transform>,
    mut materials: Query<&mut Material>,
) {
    data.timer.tick(time.delta());

    let t = time.elapsed_seconds();

    // ------------------ //
    // ---- l1,l2,l3 ---- //

    let lights = [data.l1, data.l2, data.l3].into_iter().enumerate();

    for (light_id, light) in lights {
        let mut xform = xforms.get_mut(light).unwrap();

        xform.translation.y = 2.0 + (t * 2.5 + (light_id as f32)).cos() * 1.8;
    }

    // ------------ //
    // ---- d1 ---- //

    let mut xform = xforms.get_mut(data.d1).unwrap();
    let mut material = materials.get_mut(data.d1).unwrap();

    xform.translation.y = 1.5 + (t * 2.0).sin() / 2.0;

    xform.rotation =
        Quat::from_rotation_z(t.sin()) * Quat::from_rotation_y(t.cos());

    let color = material.color.as_mut().unwrap();

    color.r = t.sin().abs();
    color.g = t.cos().abs();
    color.b = (t.sin() + t.cos()).abs();

    // ------------ //
    // ---- d2 ---- //

    if data.timer.just_finished() {
        // if let Some(d2) = data.d2.take() {
        //     commands.entity(d2).despawn();
        // } else {
        //     let d2 = commands
        //         .model("diamond")
        //         .dynamic()
        //         .with_translation(vec3(0.0, 0.5, 2.5))
        //         .with_scale(Vec3::splat(0.4))
        //         .with_material(
        //             Material::default()
        //                 .with_color(Color {
        //                     r: 1.0,
        //                     g: 0.0,
        //                     b: 0.0,
        //                 })
        //                 .without_texture(),
        //         )
        //         .spawn()
        //         .id();

        //     data.d2 = Some(d2);
        // }
    }
}
