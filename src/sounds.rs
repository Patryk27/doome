use bevy::prelude::*;
use doome_bevy::prelude::Body;

use crate::bullets::DamageDealt;
use crate::prelude::*;
use crate::rng::RngState;

const MIN_TIME_BETWEEN_IDLE_SOUNDS: f32 = 1.0;
const TIME_BETWEEN_FOOTSTEPS: f32 = 0.5;
const IDLE_SOUND_CHANCE: f32 = 0.005;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
        app.add_system(react_to_damage);
        app.add_system(footsteps);
        app.add_system(enemy_idle_noise);
    }
}

#[derive(Resource)]
struct Sounds {
    enemy_damage: Handle<AudioSource>,
    enemy_idle_sound: Handle<AudioSource>,
    player_damage: Handle<AudioSource>,
    footstep: Handle<AudioSource>,
}

#[derive(Resource)]
struct SoundsState {
    timer: f32,
    enemy_idle_sounds_timer: f32,
}

fn init(mut commands: Commands, assets: Res<AssetServer>) {
    let enemy_damage: Handle<AudioSource> = assets.load("audio/enemy_dmg.wav");
    let enemy_idle_sound: Handle<AudioSource> =
        assets.load("audio/enemy_idle_noise.wav");
    let player_damage: Handle<AudioSource> =
        assets.load("audio/player_dmg.wav");
    let footstep: Handle<AudioSource> = assets.load("audio/footstep.wav");

    commands.insert_resource(SoundsState {
        timer: 0.0,
        enemy_idle_sounds_timer: 0.0,
    });

    commands.insert_resource(Sounds {
        enemy_damage,
        enemy_idle_sound,
        player_damage,
        footstep,
    });
}

fn enemy_idle_noise(
    time: Res<Time>,
    mut state: ResMut<SoundsState>,
    sounds: Res<Sounds>,
    mut rng: ResMut<RngState>,
    audio: Res<Audio>,
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
            audio.play(sounds.enemy_idle_sound.clone());

            state.enemy_idle_sounds_timer = 0.0;
        }
    }
}

fn footsteps(
    time: Res<Time>,
    mut state: ResMut<SoundsState>,
    sounds: Res<Sounds>,
    audio: Res<Audio>,
    player: Query<(&Player, &Body)>,
) {
    let (_, body) = player.single();

    let delta = time.delta_seconds();
    state.timer += delta;

    if state.timer > TIME_BETWEEN_FOOTSTEPS {
        state.timer = 0.0;

        if body.velocity.length() > 0.01 {
            audio.play(sounds.footstep.clone());
        }
    }
}

fn react_to_damage(
    sounds: Res<Sounds>,
    audio: ResMut<Audio>,
    mut damages_dealt: EventReader<DamageDealt>,
    enemies: Query<&Enemy>,
    player: Query<&Player>,
) {
    for damage in damages_dealt.iter() {
        if enemies.get(damage.entity).is_ok() {
            audio.play(sounds.enemy_damage.clone());
        }

        if player.get(damage.entity).is_ok() {
            audio.play(sounds.player_damage.clone());
        }
    }
}
