use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::health::Health;
use doome_bevy::prelude::Player;
use doome_engine::Canvas;

const FACE_SWAY_SPEED: f32 = 2.0;

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

    let face = if health.val > 50.0 {
        smiley_face
    } else if health.val > 25.0 {
        mehey_face
    } else {
        angrey_face
    };

    canvas.blit(0, 0, assets.image(ui_image));

    let y = if (FACE_SWAY_SPEED * time.elapsed_seconds()).sin() > 0.0 {
        0
    } else {
        1
    };
    canvas.blit(0, y, assets.image(face));
}
