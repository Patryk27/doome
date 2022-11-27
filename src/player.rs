use bevy::input::mouse::MouseMotion;
use doome_bevy::convert::graphical_to_physical;

use crate::commands::{Command, Item};
use crate::prelude::*;
use crate::weapons::{PrefabWeapons, Weapon};

const MOUSE_ROTATION_SENSITIVITY: f32 = 0.15;
const PLANAR_MOVEMENT_SPEED: f32 = 7.5;
const ROTATION_SPEED: f32 = 2.0;
const ACCELERATION_SPEED: f32 = 8.0;
const BRAKING_SPEED: f32 = 24.0;
const SWAY_FREQ: f32 = 4.0;
const MAX_SWAY: f32 = 0.3;

pub struct PlayerPlugin;

pub struct PlayerShot;

pub struct AddScreenShake(pub f32);

#[derive(Resource)]
struct ScreenShake(f32);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerShot>();
        app.add_event::<AddScreenShake>();

        app.insert_resource(ScreenShake(0.0));

        app.add_startup_system(spawn);

        app.add_system(process_movement);
        app.add_system(handle_shooting);
        app.add_system(sync_camera.after(process_movement));
        app.add_system(update_screen_shake);
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
        Collider::circle(0.35, 16),
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
    let (player, mut body, mut transform) = player.single_mut();
    let delta = time.delta_seconds();

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

fn sync_camera(
    time: Res<Time>,
    screenshake: Res<ScreenShake>,
    mut camera: Query<&mut Camera>,
    player: Query<(&Transform, &Body), With<Player>>,
) {
    let Ok(mut camera) = camera.get_single_mut() else { return };
    let Ok((transform, body)) = player.get_single() else { return };
    let pos = transform.translation;

    let sway =
        camera_sway(body.velocity.length(), time.elapsed_seconds() * SWAY_FREQ);
    let screen_shake = screen_shake(time.elapsed_seconds(), screenshake.0);

    let camera_effects =
        map_to_player_transform(sway + screen_shake, transform);

    camera.origin = camera_effects + vec3(pos.x, 1.2, pos.z);
    camera.look_at = camera.origin + transform.forward() * 5.0;
}

fn map_to_player_transform(v: Vec2, transform: &Transform) -> Vec3 {
    let x = transform.right() * v.x;
    let z = transform.up() * v.y;

    x + z
}

fn screen_shake(time: f32, factor: f32) -> Vec2 {
    let x = (time * 123.0).sin() * factor;
    let y = (time * 164.0).cos() * factor;

    Vec2::new(x, y)
}

fn update_screen_shake(
    mut increments: EventReader<AddScreenShake>,
    time: Res<Time>,
    mut screen_shake: ResMut<ScreenShake>,
) {
    let mut increment = 0.0;

    for ev in increments.iter() {
        increment += ev.0;
    }

    screen_shake.0 = (screen_shake.0 + increment).clamp(0.0, 1.0);
    screen_shake.0 = (screen_shake.0 - time.delta_seconds()).clamp(0.0, 1.0);
}

fn camera_sway(velocity: f32, time: f32) -> Vec2 {
    let sway_factor =
        crate::math::remap(velocity, 0.0, PLANAR_MOVEMENT_SPEED, 0.0, MAX_SWAY);

    // This makes a sideways figure 8
    // https://www.wolframalpha.com/input?key=&i=x+%3D+sin%28t%29%2C+y+%3D+sin%28t%29+*+cos%28t%29
    vec2(time.sin(), time.cos() * time.sin()) * sway_factor
}

pub fn handle_shooting(
    input_lock: Res<InputLock>,
    mut commands: Commands,
    mut game_commands: EventWriter<Command>,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut weapon: Query<(&Player, &Transform, &mut Weapon)>,
    mut shots: EventWriter<PlayerShot>,
) {
    let (_, transform, mut weapon) = weapon.single_mut();

    if input_lock.is_locked {
        return;
    }

    if weapon.out_of_ammo() {
        game_commands.send(Command::Give {
            what: Item::Handgun,
        });

        return;
    }

    if !weapon.can_shoot() {
        return;
    }

    if mouse.pressed(MouseButton::Left) || keys.pressed(KeyCode::Space) {
        weapon.shoot(&mut commands, &transform, transform.forward());
        shots.send(PlayerShot);
    }
}
