use std::time::Duration;

use bevy::prelude::*;
use doome_bevy::assets::{AssetHandle, Assets};
use doome_bevy::audio::Audio;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::health::Health;
use doome_bevy::player::Player;
use doome_engine::Canvas;
use image::RgbaImage;

use crate::InputLock;

const GUN_OFFSET_Y: u16 = 16;

#[derive(Resource)]
pub struct Gun {
    gun_idle: AssetHandle<RgbaImage>,
    gun_fire_sequence: Vec<AssetHandle<RgbaImage>>,
}

#[derive(Component)]
pub struct ShootingAnimation {
    is_firing: bool,
    current_frame: usize,
    num_frames: usize,
    timer: Timer,
}

pub fn setup(mut commands: Commands, assets: Res<Assets>) {
    let gun_idle = assets.load_image("rifle_0");

    let gun_fire_sequence =
        vec![assets.load_image("rifle_1"), assets.load_image("rifle_2")];

    commands.insert_resource(Gun {
        gun_idle,
        gun_fire_sequence,
    });

    let mut timer =
        Timer::new(Duration::from_millis(100), TimerMode::Repeating);

    timer.pause();

    commands.spawn(ShootingAnimation {
        is_firing: false,
        current_frame: 0,
        num_frames: 2,
        timer,
    });
}

pub fn render(
    time: Res<Time>,
    assets: Res<Assets>,
    mut renderer: ResMut<DoomeRenderer>,
    data: Res<Gun>,
    shooting_anim: Query<&ShootingAnimation>,
    player: Query<(&Player, &Health)>,
) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(frame);

    let (player, _) = player.single();

    let (sway_x, sway_y) = if player.can_move {
        calc_sway(time.elapsed_seconds())
    } else {
        calc_sway(0.0)
    };

    let shooting_anim = shooting_anim.single();

    let gun_image = if shooting_anim.is_firing {
        data.gun_fire_sequence[shooting_anim.current_frame]
    } else {
        data.gun_idle
    };

    canvas.blit(sway_x, sway_y - GUN_OFFSET_Y, assets.image(gun_image));
}

// TODO: Attach an event writer
pub fn trigger_shoot(
    input_lock: Res<InputLock>,
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<Assets>,
    mut player: Query<(&mut Player, &Transform)>,
    mut shooting_animation: Query<&mut ShootingAnimation>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut audio: ResMut<Audio>,
) {
    let (mut player, player_transform) = player.single_mut();

    if !player.can_move {
        return;
    }

    let delta = time.delta_seconds();

    player.shooter.update(delta);

    if input_lock.is_locked {
        return;
    }

    if !player.shooter.can_shoot() {
        return;
    }

    let mut shooting_animation = shooting_animation.single_mut();

    if mouse.pressed(MouseButton::Left) || keyboard.pressed(KeyCode::Space) {
        audio.play(assets.load_sound("gun_shoot"));
        shooting_animation.is_firing = true;
        shooting_animation.current_frame = 0;
        shooting_animation.timer.unpause();
        shooting_animation.timer.reset();

        player.shooter.shoot(player_transform, &mut commands);
    }
}

pub fn update_gun(
    time: Res<Time>,
    mut shooting_animation: Query<&mut ShootingAnimation>,
) {
    let mut shooting_animation = shooting_animation.single_mut();
    shooting_animation.timer.tick(time.delta());

    if shooting_animation.is_firing && shooting_animation.timer.just_finished()
    {
        shooting_animation.current_frame += 1;

        // TODO: No magic number
        if shooting_animation.current_frame == shooting_animation.num_frames {
            shooting_animation.current_frame = 0;
            shooting_animation.is_firing = false;
            shooting_animation.timer.pause();
        } else {
            shooting_animation.timer.reset();
        }
    }
}

fn calc_sway(t: f32) -> (u16, u16) {
    const MAX_SWAY: f32 = 8.0;

    let y = MAX_SWAY - (t * 1.2).cos() * MAX_SWAY;

    (0 as _, y as _)
}
