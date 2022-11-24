use bevy::input::mouse::MouseMotion;
use doome_bevy::convert::graphical_to_physical;

use crate::prelude::*;

pub fn spawn(mut commands: Commands) {
    let shooter = Shooter::default()
        .with_speed(20.0)
        .with_cooldown(0.2)
        .with_damage(30.0);

    commands.spawn((
        Player::new(shooter),
        Transform::default(),
        Body {
            velocity: Vec2::ZERO,
            body_type: BodyType::Kinematic,
        },
        Collider::circle(0.5),
        Health::new(100.0),
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

    let (player, mut body, mut transform) = player.single_mut();
    let delta = time.delta_seconds();

    if input_lock.is_locked {
        return;
    }

    // TODO a bit wonky
    #[cfg(not(target_arch = "wasm32"))]
    for ev in mouse_motion.iter() {
        transform.rotate_axis(
            Vec3::Y,
            MOUSE_ROTATION_SENSITIVITY * ev.delta.x * delta,
        );
    }

    body.velocity = Vec2::ZERO;

    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::S) {
        let sign = if keys.pressed(KeyCode::W) { 1.0 } else { -1.0 };
        body.velocity += graphical_to_physical(transform.forward() * sign);
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
        body.velocity += graphical_to_physical(transform.left() * sign);
    }

    body.velocity = body.velocity.normalize_or_zero() * PLANAR_MOVEMENT_SPEED;

    if !player.can_move {
        body.velocity = Default::default();
    }
}
