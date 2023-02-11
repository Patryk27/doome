use bevy::prelude::*;
use doome_bevy::assets::DoomeAssets;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::health::Health;
use doome_bevy::prelude::Player;
use doome_engine::Canvas;

use super::{UiState, HEALTHY_THRESHOLD, WOUNDED_THRESHOLD};
use crate::prelude::LevelsCoordinator;

const FACE_SWAY_SPEED: f32 = 2.0;
const ANGRY_JITTER_SPEED: (f32, f32) = (128.7, 120.13);

enum State {
    Happy,
    Meh,
    Angrey,
}

pub fn render(
    time: Res<Time>,
    assets: Res<DoomeAssets>,
    player: Query<(&Player, &Health)>,
    gun_state: Res<super::gun::State>,
    mut renderer: ResMut<DoomeRenderer>,
    ui: Res<UiState>,
    levels_coordinator: Res<LevelsCoordinator>,
) {
    if !ui.hud_visible || levels_coordinator.is_game_over {
        return;
    }

    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(frame);

    let ui_image = assets.load_image("ui_static");

    let (_player, health) = player.single();

    let smiley_face = assets.load_image("smiley");
    let mehey_face = assets.load_image("mehey");
    let angrey_face = assets.load_image("angrey");

    let state = if health.health > HEALTHY_THRESHOLD {
        State::Happy
    } else if health.health > WOUNDED_THRESHOLD {
        State::Meh
    } else {
        State::Angrey
    };

    let face = match state {
        State::Happy => smiley_face,
        State::Meh => mehey_face,
        State::Angrey => angrey_face,
    };

    canvas.blit(0, 0, assets.image(ui_image));

    canvas.blit(0, 0, assets.image(gun_state.current_weapon.ui_icon));

    let seconds = time.elapsed_seconds();
    let (x, y) = match state {
        State::Happy => {
            let y = if (FACE_SWAY_SPEED * seconds).sin() > 0.0 {
                0
            } else {
                1
            };

            (0, y)
        }
        State::Meh => {
            let x = if (FACE_SWAY_SPEED * seconds).sin() > 0.0 {
                0
            } else {
                1
            };

            (x, 0)
        }
        State::Angrey => {
            let y = if (seconds * ANGRY_JITTER_SPEED.0).sin() > 0.0 {
                0
            } else {
                1
            };

            let x = if ((seconds + 42.0) * ANGRY_JITTER_SPEED.1).sin() > 0.0 {
                0
            } else {
                1
            };

            (x, y)
        }
    };

    canvas.blit(x, y, assets.image(face));
}
