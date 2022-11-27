use bevy::prelude::*;
use doome_bevy::audio::{Audio, Sound};
use doome_bevy::prelude::{AssetHandle, Assets, Body};

use crate::bullets::DamageDealt;
use crate::prelude::*;
use crate::rng::RngState;

const MIN_TIME_BETWEEN_IDLE_SOUNDS: f32 = 1.0;
const TIME_BETWEEN_FOOTSTEPS: f32 = 0.5;
const IDLE_SOUND_CHANCE: f32 = 0.005;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        let assets = app.world.resource::<Assets>();

        let enemy_damage = assets.load_sound("enemy_dmg");
        let enemy_idle_sound = assets.load_sound("enemy_idle_noise");
        let player_damage = assets.load_sound("player_dmg");
        let footstep = assets.load_sound("footstep");

        app.insert_resource(Sounds {
            enemy_damage,
            enemy_idle_sound,
            player_damage,
            footstep,
        });
        app.insert_resource(SoundsState {
            timer: 0.0,
            enemy_idle_sounds_timer: 0.0,
        });
        app.add_system(react_to_damage);
        app.add_system(footsteps);
        app.add_system(enemy_idle_noise);
    }
}

#[derive(Resource)]
struct Sounds {
    enemy_damage: AssetHandle<Sound>,
    enemy_idle_sound: AssetHandle<Sound>,
    player_damage: AssetHandle<Sound>,
    footstep: AssetHandle<Sound>,
}

#[derive(Resource)]
struct SoundsState {
    timer: f32,
    enemy_idle_sounds_timer: f32,
}

fn enemy_idle_noise(
    time: Res<Time>,
    mut state: ResMut<SoundsState>,
    sounds: Res<Sounds>,
    mut rng: ResMut<RngState>,
    mut audio: ResMut<Audio>,
    enemies: Query<&Enemy>,
) {
    if enemies.iter().count() == 0 {
        return;
    }

    let delta = time.delta_seconds();

    state.enemy_idle_sounds_timer += delta;

    if state.enemy_idle_sounds_timer > MIN_TIME_BETWEEN_IDLE_SOUNDS {
        let r = rng.gen::<f32>();

        if r < IDLE_SOUND_CHANCE {
            audio.play(sounds.enemy_idle_sound);

            state.enemy_idle_sounds_timer = 0.0;
        }
    }
}

fn footsteps(
    time: Res<Time>,
    mut state: ResMut<SoundsState>,
    sounds: Res<Sounds>,
    mut audio: ResMut<Audio>,
    player: Query<(&Player, &Body)>,
) {
    let (_, body) = player.single();

    let delta = time.delta_seconds();
    state.timer += delta;

    if state.timer > TIME_BETWEEN_FOOTSTEPS {
        state.timer = 0.0;

        if body.velocity.length() > 0.01 {
            audio.play(sounds.footstep);
        }
    }
}

fn react_to_damage(
    sounds: Res<Sounds>,
    mut audio: ResMut<Audio>,
    mut damages_dealt: EventReader<DamageDealt>,
    enemies: Query<&Enemy>,
    player: Query<&Player>,
) {
    for damage in damages_dealt.iter() {
        if enemies.get(damage.entity).is_ok() {
            audio.play(sounds.enemy_damage);
        }

        if player.get(damage.entity).is_ok() {
            audio.play(sounds.player_damage);
        }
    }
}
