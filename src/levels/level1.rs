use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::components::*;
use doome_bevy::physics::{Body, BodyType, CircleCollider, Collider, RayCast};
use glam::{vec3, Vec3Swizzles};

use super::utils::*;
use crate::interaction::TextInteraction;
use crate::markers::{FollowPlayerAbove, InteractableHighlight};

pub fn init(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        Player,
        // Transform::IDENTITY,
        Transform::from_rotation(Quat::from_rotation_x(PI)),
        Body {
            velocity: vec3(0.0, 0.0, 0.0),
            body_type: BodyType::Kinematic,
        },
        Collider::Circle(CircleCollider { radius: 0.5 }),
        // TODO: How to have multiple raycasts on the same entity?
        RayCast {
            origin: Vec2::NEG_Y * 0.6, // A little in front of our own collider
            direction: Vec2::NEG_Y * 120.0,
            hit: None,
        },
    ));

    commands.floor(-6, -3, 6, 15 + 3);
    commands.wall(-6, -3, -6, 15 + 3, 3);
    commands.wall(6, -3, 6, 15 + 3, 1);
    // commands.wall(1, 3, 3, 3, 0);
    // commands.wall(3, 3, 3, -3, 1);
    // commands.wall(-3, -3, 3, -3, 2);
    // commands.wall(-3, -3, -3, 3, 3);
    // commands.floor(-1, 3, 1, 5);
    // commands.wall(-1, 5, 1, 5, 0);
    // commands.wall(1, 3, 1, 5, 1);
    // commands.wall(-1, 3, -1, 5, 3);
    // commands.ceiling(-10, -10, 10, 10);

    let column = assets.load_model("column");
    let chandelier = assets.load_model("chandelier");
    let thingy = assets.load_model("thingy");
    let monke = assets.load_model("monke");
    let diamond = assets.load_model("diamond");

    let column_positions = (0..5).map(|n| n as f32 * 3.0);
    const X_OFFSET: f32 = 4.0;

    for z in column_positions {
        let left_column_pos = vec3(-X_OFFSET, 0.0, z);
        let right_column_pos = vec3(X_OFFSET, 0.0, z);

        commands
            .model(column)
            .with_translation(left_column_pos)
            .spawn()
            .insert(Collider::Circle(CircleCollider { radius: 0.5 }));

        commands
            .model(column)
            .with_translation(right_column_pos)
            .spawn()
            .insert(Collider::Circle(CircleCollider { radius: 0.5 }));
    }

    commands
        .model(chandelier)
        .with_translation(vec3(0.0, 6.0, 7.5))
        .with_material(
            Material::default().with_reflectivity(1.0).without_texture(),
        )
        .spawn();

    commands
        .model(thingy)
        .with_translation(vec3(0.0, 0.0, 7.5))
        .with_material(
            Material::default().with_reflectivity(1.0).without_texture(),
        )
        .spawn();

    // let l1 = commands.point_light(-1.5, 3.0, -1.0, 0.8, 0.0, 0.0).id();
    // let l2 = commands.point_light(0.0, 3.0, -1.0, 0.0, 0.8, 0.0).id();
    // let l3 = commands.point_light(1.5, 3.0, -1.0, 0.0, 0.0, 0.8).id();

    // Main "ambient" light
    const AMBIENT: f32 = 0.2;
    commands.point_light(0.0, 4.0, 4.0, 0.6, AMBIENT, AMBIENT);
    commands.point_light(0.0, 2.0, -4.0, AMBIENT, AMBIENT, 0.6);

    let middle_monke_pos = vec3(0.0, 1.0, 11.0);

    commands
        .spot_light(
            vec3(0.0, 4.0, 7.5),
            middle_monke_pos,
            PI / 12.0,
            vec3(1.0, 0.0, 0.0),
        )
        .insert(InteractableHighlight);

    commands
        .model(monke)
        .with_translation(middle_monke_pos)
        .with_scale(Vec3::splat(0.5))
        .spawn()
        .insert(Collider::Circle(CircleCollider { radius: 0.5 }))
        .insert(TextInteraction {
            content: format!("Middle Monke"),
        });

    // commands
    //     .model("diamond")
    //     .dynamic()
    //     .with_translation(vec3(0.0, 0.0, 0.0))
    //     .with_scale(Vec3::splat(0.4))
    //     .with_material(
    //         Material::default()
    //             .with_color(Color {
    //                 r: 1.0,
    //                 g: 1.0,
    //                 b: 1.0,
    //             })
    //             .with_reflectivity(1.0)
    //             .without_texture(),
    //     )
    //     .spawn()
    //     .insert(PlayerRayCastMarker);

    commands
        .point_light(0.0, 2.0, -4.0, AMBIENT, AMBIENT, AMBIENT)
        .insert(FollowPlayerAbove { offset: 1.0 });

    let d1 = commands
        .model(diamond)
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
        // l1,
        // l2,
        // l3,
        d1,
        d2: None,
        timer: Timer::new(Duration::from_millis(1500), TimerMode::Repeating),
    });
}

#[derive(Resource)]
pub struct Data {
    // l1: Entity,
    // l2: Entity,
    // l3: Entity,
    d1: Entity,
    d2: Option<Entity>,
    timer: Timer,
}

#[derive(Component)]
pub struct PlayerRayCastMarker;

pub fn sync_raycast_marker(
    player: Query<(&Player, &RayCast)>,
    mut marker: Query<(&PlayerRayCastMarker, &mut Transform)>,
) {
    let Ok((_, raycast)) = player.get_single() else { return };
    let Some(raycast_hit) = raycast.hit.as_ref() else { return };

    let Ok((_, mut marker_transform)) = marker.get_single_mut() else { return };

    marker_transform.translation = raycast_hit.position.extend(0.0).xzy();
}

pub fn log_player_position(
    mut last_logged_at: Local<f32>,
    time: Res<Time>,
    player: Query<(&Player, &Transform)>,
) {
    const LOG_INTERVAL: f32 = 1.0; // in seconds

    let Ok((_, transform)) = player.get_single() else { return };

    if time.elapsed_seconds() - *last_logged_at > LOG_INTERVAL {
        info!("Player position: {:?}", transform.translation);
        info!(
            "Player orientation (euler XYZ): {:?}",
            transform.rotation.to_euler(EulerRot::XYZ)
        );
        *last_logged_at = time.elapsed_seconds();
    }
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

    // let lights = [data.l1, data.l2, data.l3].into_iter().enumerate();

    // for (light_id, light) in lights {
    //     let mut xform = xforms.get_mut(light).unwrap();

    //     xform.translation.y = 2.0 + (t * 2.5 + (light_id as f32)).cos() * 1.8;
    // }

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
