use std::sync::Arc;

use bevy::prelude::*;
use doome_bevy::assets::DoomeAssets;
use doome_bevy::audio::DoomeAudio;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::health::Health;
use doome_bevy::player::Player;
use doome_engine::Canvas;

use super::UiState;
use crate::player::PlayerShot;
use crate::prelude::LevelsCoordinator;
use crate::weapons::{PrefabWeapons, Weapon, WeaponSprites};

const GUN_OFFSET_Y: u16 = 16;

#[derive(Resource)]
pub struct State {
    pub current_weapon: Arc<WeaponSprites>,
    pub anim_state: AnimationState,
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
            anim_state: AnimationState {
                is_firing: false,
                current_frame: 0,
                timer: 0.0,
            },
        }
    }
}

pub fn render(
    time: Res<Time>,
    assets: Res<AssetServer>,
    doome_assets: Res<DoomeAssets>,
    mut state: ResMut<State>,
    mut renderer: ResMut<DoomeRenderer>,
    player: Query<(&Player, &Health, &Weapon)>,
    audio: Res<Audio>,
    mut shots: EventReader<PlayerShot>,
    ui: Res<UiState>,
    levels_coordinator: Res<LevelsCoordinator>,
) {
    if !ui.hud_visible || levels_coordinator.is_game_over {
        return;
    }

    let State {
        current_weapon,
        anim_state: shooting_anim,
    } = state.as_mut();

    if shots.iter().count() > 0 {
        trigger_animation(&assets, shooting_anim, &audio);
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
        let frames = &current_weapon.animation;

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

    canvas.blit(sway_x, sway_y - GUN_OFFSET_Y, doome_assets.image(gun_image));
}

pub fn trigger_animation(
    assets: &AssetServer,
    shooting_animation: &mut AnimationState,
    audio: &Audio,
) {
    audio.play(assets.load("audio/gun_shoot.wav"));
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
