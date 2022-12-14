use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::audio::Audio;
use doome_bevy::billboard::Billboard;
use doome_bevy::components::{GeometryType, Material};
use doome_bevy::model_animation::{ModelAnimation, ModelAnimationFrame};

#[derive(Component)]
pub struct Explosion;

pub fn spawn_explosion(
    commands: &mut Commands,
    assets: &Assets,
    audio: &mut Audio,
    transform: Transform,
) {
    let frames = (0..=11)
        .map(|n| format!("explosion_{n}"))
        .map(|name| assets.load_model(&name))
        .map(|handle| ModelAnimationFrame {
            duration: 0.07,
            handle,
        })
        .collect();

    let model_animation = ModelAnimation {
        frames,
        frame_time: 0.0,
        current_frame: 0,
        looped: false,
        playing: true,
    };

    let starting_model = model_animation.frames[5].handle;

    audio.play(assets.load_sound("explosion"));

    commands.spawn((
        Explosion,
        transform,
        model_animation,
        starting_model,
        GeometryType::Dynamic,
        Billboard,
        Material::default().with_uv_transparency().emissive(),
    ));
}

pub fn update(
    mut commands: Commands,
    explosions: Query<(Entity, &Explosion, &ModelAnimation)>,
) {
    for (entity, _, anim) in explosions.iter() {
        if anim.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
