use bevy::prelude::*;
use doome_bevy::audio::{Audio, Sound};
use doome_bevy::prelude::{AssetHandle, Assets};

use crate::bullets::DamageDealt;
use crate::prelude::Enemy;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        let assets = app.world.resource::<Assets>();

        let enemy_damage = assets.load_sound("enemy_dmg");

        app.insert_resource(Sounds { enemy_damage });
        app.add_system(react_to_damage);
    }
}

#[derive(Resource)]
struct Sounds {
    enemy_damage: AssetHandle<Sound>,
}

fn react_to_damage(
    sounds: Res<Sounds>,
    mut audio: ResMut<Audio>,
    mut damages_dealt: EventReader<DamageDealt>,
    enemies: Query<&Enemy>,
    // player: Query<&Player>,
) {
    for damage in damages_dealt.iter() {
        if enemies.get(damage.entity).is_ok() {
            audio.play(sounds.enemy_damage);
        }
    }
}
