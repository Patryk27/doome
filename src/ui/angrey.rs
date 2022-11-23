use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::doome::DoomeRenderer;
use doome_engine::Canvas;

pub fn render(assets: Res<Assets>, mut renderer: ResMut<DoomeRenderer>) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(frame);

    let ui_image = assets.load_image("ui_static");
    let angrey_face = assets.load_image("angrey");
    canvas.blit(0, 0, assets.image(ui_image));
    canvas.blit(0, 0, assets.image(angrey_face));
}
