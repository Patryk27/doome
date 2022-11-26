use bevy::prelude::*;
use doome_bevy::audio::{Audio, Sound};
use doome_bevy::prelude::{AssetHandle, Assets, Body};

use crate::bullets::DamageDealt;
use crate::prelude::*;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        let assets = app.world.resource::<Assets>();

        let enemy_damage = assets.load_sound("enemy_dmg");
        let player_damage = assets.load_sound("player_dmg");
        let footstep = assets.load_sound("footstep");

        app.insert_resource(Sounds {
            enemy_damage,
            player_damage,
            footstep,
        });
        app.insert_resource(FootstepsState { timer: 0.0 });
        app.add_system(react_to_damage);
        app.add_system(footsteps);
    }
}

#[derive(Resource)]
struct Sounds {
    enemy_damage: AssetHandle<Sound>,
    player_damage: AssetHandle<Sound>,
    footstep: AssetHandle<Sound>,
}

#[derive(Resource)]
struct FootstepsState {
    timer: f32,
}

fn footsteps(
    time: Res<Time>,
    mut footsteps_state: ResMut<FootstepsState>,
    sounds: Res<Sounds>,
    mut audio: ResMut<Audio>,
    player: Query<(&Player, &Body)>,
) {
    let (_, body) = player.single();

    let delta = time.delta_seconds();
    footsteps_state.timer += delta;

    if footsteps_state.timer > 0.5 {
        footsteps_state.timer = 0.0;

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
