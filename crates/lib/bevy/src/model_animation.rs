use bevy::prelude::*;

use crate::assets::{AssetHandle, Model};

pub fn animate(
    time: Res<Time>,
    mut anims: Query<(&mut ModelAnimation, &mut AssetHandle<Model>)>,
) {
    for (mut anim, mut model) in anims.iter_mut() {
        anim.frame_time += time.delta_seconds();

        let current_frame = &anim.frames[anim.current_frame];

        if anim.frame_time > current_frame.duration {
            anim.frame_time = 0.0;
            anim.current_frame = (anim.current_frame + 1) % anim.frames.len();
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
}

pub struct ModelAnimationFrame {
    pub duration: f32,
    pub handle: AssetHandle<Model>,
}
