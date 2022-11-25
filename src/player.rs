use bevy::input::mouse::MouseMotion;
use doome_bevy::convert::graphical_to_physical;

use crate::prelude::*;
use crate::weapons::{PrefabWeapons, Weapon};

pub struct PlayerPlugin;

pub struct PlayerShot;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerShot>();
        app.add_startup_system(spawn);
        app.add_system(process_movement);
        app.add_system(handle_shooting);
        app.add_system(sync_camera.after(process_movement));
    }
}

pub fn spawn(mut commands: Commands, prefab_weapons: Res<PrefabWeapons>) {
    commands.spawn((
        Player::new(),
        Transform::default(),
        Body {
            acceleration: Vec2::ZERO,
            velocity: Vec2::ZERO,
            body_type: BodyType::Kinematic,
        },
        Weapon::new(prefab_weapons.handgun.0.clone()),
        Collider::circle(0.5, 16),
        Health::new(100.0, 100.0),
    ));
}

pub fn process_movement(
    input_lock: Res<InputLock>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<(&Player, &mut Body, &mut Transform)>,
) {
    const MOUSE_ROTATION_SENSITIVITY: f32 = 0.5;
    const PLANAR_MOVEMENT_SPEED: f32 = 7.5;
    const ROTATION_SPEED: f32 = 2.0;
    const ACCELERATION_SPEED: f32 = 8.0;
    const BRAKING_SPEED: f32 = 12.0;

    let (player, mut body, mut transform) = player.single_mut();
    let delta = time.delta_seconds();

    // TODO a bit wonky
    #[cfg(not(target_arch = "wasm32"))]
    for ev in mouse_motion.iter() {
        transform.rotate_axis(
            Vec3::Y,
            MOUSE_ROTATION_SENSITIVITY * ev.delta.x * delta,
        );
    }

    let mut desired_velocity = Vec2::ZERO;

    body.acceleration = Vec2::ZERO;

    if !input_lock.is_locked {
        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::S) {
            let sign = if keys.pressed(KeyCode::W) { 1.0 } else { -1.0 };
            desired_velocity +=
                graphical_to_physical(transform.forward() * sign);
        }

        if keys.pressed(KeyCode::Comma) || keys.pressed(KeyCode::Period) {
            let sign = if keys.pressed(KeyCode::Comma) {
                -1.0
            } else {
                1.0
            };

            transform.rotate_axis(Vec3::Y, ROTATION_SPEED * sign * delta);
        }

        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::D) {
            let sign = if keys.pressed(KeyCode::A) { -1.0 } else { 1.0 };
            desired_velocity += graphical_to_physical(transform.left() * sign);
        }
    }

    let desired_velocity =
        desired_velocity.normalize_or_zero() * PLANAR_MOVEMENT_SPEED;

    if desired_velocity.length() > 0.0 {
        let diff = desired_velocity - body.velocity;
        body.acceleration = diff * ACCELERATION_SPEED;
    } else {
        body.acceleration =
            -body.velocity.clamp_length_max(1.0) * BRAKING_SPEED;
    }

    if !player.can_move {
        body.acceleration = Vec2::ZERO;
        body.velocity = Vec2::ZERO;
    }
}

pub fn sync_camera(
    mut camera: Query<&mut Camera>,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(mut camera) = camera.get_single_mut() else { return };
    let Ok(transform) = player.get_single() else { return };
    let pos = transform.translation;

    camera.origin = vec3(pos.x, 1.2, pos.z);
    camera.look_at = camera.origin + transform.forward() * 5.0;
}

pub fn handle_shooting(
    input_lock: Res<InputLock>,
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut weapon: Query<(&Player, &Transform, &mut Weapon)>,
    mut shots: EventWriter<PlayerShot>,
) {
    let (_, transform, mut weapon) = weapon.single_mut();

    if input_lock.is_locked {
        return;
    }

    if !weapon.can_shoot() {
        return;
    }

    if mouse.pressed(MouseButton::Left) || keys.pressed(KeyCode::Space) {
        weapon.shoot(&mut commands, &transform);
        shots.send(PlayerShot);
    }
}
