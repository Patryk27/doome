use bevy::prelude::*;
use doome_bevy::assets::DoomeAssets;
use doome_bevy::health::Death;

use crate::enemies::Enemy;
use crate::explosions::spawn_explosion;
use crate::player::AddScreenShake;

pub struct CharonPlugin;

impl Plugin for CharonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_enemy_deaths);
    }
}

fn handle_enemy_deaths(
    mut commands: Commands,
    doome_assets: Res<DoomeAssets>,
    assets: Res<AssetServer>,
    audio: Res<Audio>,
    mut deaths: EventReader<Death>,
    mut screen_shakes: EventWriter<AddScreenShake>,
    enemies: Query<(&Transform, &Enemy)>,
) {
    for death in deaths.iter() {
        if let Ok((transform, _)) = enemies.get(death.0) {
            log::info!("Spawning explosion on enemy");
            let mut transform = transform.clone().with_scale(Vec3::ONE * 6.0);
            transform.translation += Vec3::Y * 1.0;

            spawn_explosion(
                &mut commands,
                &doome_assets,
                &assets,
                &audio,
                transform,
            );
            screen_shakes.send(AddScreenShake(0.5));

            commands.entity(death.0).despawn();
        }
    }
}
