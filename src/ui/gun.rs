use std::sync::Arc;
use std::time::Duration;

use bevy::prelude::*;
use doome_bevy::assets::{AssetHandle, Assets};
use doome_bevy::audio::Audio;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::health::Health;
use doome_bevy::player::Player;
use doome_engine::Canvas;
use image::RgbaImage;

use super::InputLock;
use crate::player::PlayerShot;
use crate::weapons::{PrefabWeapons, Weapon, WeaponSprites};

const GUN_OFFSET_Y: u16 = 16;

#[derive(Resource)]
pub struct State {
    current_weapon: Arc<WeaponSprites>,
}

#[derive(Component)]
pub struct AnimationState {
    is_firing: bool,
    current_frame: usize,
    timer: f32,
}

impl State {
    pub fn new(prefab_weaons: &PrefabWeapons) -> Self {
        Self {
            current_weapon: prefab_weaons.handgun.1.clone(),
        }
    }
}

pub fn setup(mut commands: Commands, assets: Res<Assets>) {
    let mut timer =
        Timer::new(Duration::from_millis(100), TimerMode::Repeating);

    timer.pause();

    commands.spawn(AnimationState {
        is_firing: false,
        current_frame: 0,
        timer: 0.0,
    });
}

pub fn render(
    time: Res<Time>,
    assets: Res<Assets>,
    state: Res<State>,
    mut renderer: ResMut<DoomeRenderer>,
    mut shooting_animation: Query<&mut AnimationState>,
    player: Query<(&Player, &Health, &Weapon)>,
    mut audio: ResMut<Audio>,
    mut shots: EventReader<PlayerShot>,
) {
    let mut shooting_anim = shooting_animation.single_mut();
    let shooting_anim = shooting_anim.as_mut();

    if shots.iter().count() > 0 {
        trigger_animation(&assets, shooting_anim, &mut audio);
    }

    update_animation(&time, shooting_anim);

    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(frame);

    let (player, _, _) = player.single();

    let (sway_x, sway_y) = if player.can_move {
        calc_sway(time.elapsed_seconds())
    } else {
        calc_sway(0.0)
    };

    let gun_image = if shooting_anim.is_firing {
        let frames = &state.current_weapon.animation;

        if shooting_anim.current_frame >= frames.len() {
            shooting_anim.current_frame = 0;
            shooting_anim.is_firing = false;

            state.current_weapon.idle
        } else {
            frames[shooting_anim.current_frame]
        }
    } else {
        state.current_weapon.idle
    };

    canvas.blit(sway_x, sway_y - GUN_OFFSET_Y, assets.image(gun_image));
}

// TODO: Attach an event writer
pub fn trigger_animation(
    assets: &Assets,
    shooting_animation: &mut AnimationState,
    audio: &mut Audio,
) {
    audio.play(assets.load_sound("gun_shoot"));
    shooting_animation.is_firing = true;
    shooting_animation.current_frame = 0;
    shooting_animation.timer = 0.0;
}

const FRAME_SWITCH_SPEED: f32 = 0.1;

pub fn update_animation(time: &Time, shooting_animation: &mut AnimationState) {
    shooting_animation.timer += time.delta_seconds();

    if shooting_animation.is_firing
        && shooting_animation.timer > FRAME_SWITCH_SPEED
    {
        shooting_animation.timer = 0.0;
        shooting_animation.current_frame += 1;
    }
}

fn calc_sway(t: f32) -> (u16, u16) {
    const MAX_SWAY: f32 = 8.0;

    let y = MAX_SWAY - (t * 1.2).cos() * MAX_SWAY;

    (0 as _, y as _)
}
