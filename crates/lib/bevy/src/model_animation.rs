use bevy::prelude::*;

use crate::assets::{DoomeAssetHandle, Model};

pub fn animate(
    time: Res<Time>,
    mut anims: Query<(&mut ModelAnimation, &mut DoomeAssetHandle<Model>)>,
) {
    for (mut anim, mut model) in anims.iter_mut() {
        if !anim.playing {
            continue;
        }

        anim.frame_time += time.delta_seconds();

        let current_frame = &anim.frames[anim.current_frame];

        if anim.frame_time > current_frame.duration {
            anim.frame_time = 0.0;
            anim.current_frame = anim.current_frame + 1;

            if anim.current_frame == anim.frames.len() {
                if anim.looped {
                    anim.current_frame = 0;
                } else {
                    anim.current_frame -= 1;
                    anim.playing = false;
                }
            }
        }

        let current_frame = &anim.frames[anim.current_frame];

        *model = current_frame.handle;
    }
}

#[derive(Component)]
pub struct ModelAnimation {
    pub frames: Vec<ModelAnimationFrame>,
    pub frame_time: f32,
    pub current_frame: usize,
    pub looped: bool,
    pub playing: bool,
}

impl ModelAnimation {
    pub fn is_finished(&self) -> bool {
        !self.playing && self.current_frame == self.frames.len() - 1
    }
}

pub struct ModelAnimationFrame {
    pub duration: f32,
    pub handle: DoomeAssetHandle<Model>,
}
