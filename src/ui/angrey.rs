use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::health::Health;
use doome_bevy::prelude::Player;
use doome_engine::Canvas;

const FACE_SWAY_SPEED: f32 = 2.0;
const ANGRY_JITTER_SPEED: (f32, f32) = (128.7, 120.13);

enum State {
    Happy,
    Meh,
    Angrey,
}

pub fn render(
    time: Res<Time>,
    assets: Res<Assets>,
    player: Query<(&Player, &Health)>,
    mut renderer: ResMut<DoomeRenderer>,
) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(frame);

    let ui_image = assets.load_image("ui_static");

    let (_player, health) = player.single();

    let smiley_face = assets.load_image("smiley");
    let mehey_face = assets.load_image("mehey");
    let angrey_face = assets.load_image("angrey");

    let state = if health.health > 50.0 {
        State::Happy
    } else if health.health > 25.0 {
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
