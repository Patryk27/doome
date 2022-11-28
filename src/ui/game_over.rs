use doome_bevy::doome::DoomeRenderer;
use doome_engine::Canvas;
use doome_surface::Color;
use image::RgbaImage;

use crate::prelude::*;

pub fn setup(mut commands: Commands, assets: Res<Assets>) {
    let you_died = assets.load_image("you_died");
    let click_to_restart = assets.load_image("click_to_restart");

    commands.insert_resource(GameOverAssets {
        img1: you_died,
        img2: click_to_restart,
    });
}

#[derive(Resource)]
pub struct GameOverAssets {
    img1: AssetHandle<RgbaImage>,
    img2: AssetHandle<RgbaImage>,
}

pub fn render(
    assets: Res<Assets>,
    game_over_assets: Res<GameOverAssets>,
    levels_coordinator: Res<LevelsCoordinator>,
    mut renderer: ResMut<DoomeRenderer>,
) {
    if !levels_coordinator.is_game_over {
        return;
    }

    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(frame);

    let img1 = assets.image(game_over_assets.img1);

    let alpha1 =
        ((levels_coordinator.time_in_game_over - 0.5) * 255.0).min(255.0) as u8;

    let img2 = assets.image(game_over_assets.img2);

    let alpha2 =
        ((levels_coordinator.time_in_game_over - 1.8) * 255.0).min(255.0) as u8;

    canvas.blit_blended(0, 0, img1, Color::WHITE.with_a(alpha1));
    canvas.blit_blended(0, 0, img2, Color::WHITE.with_a(alpha2));
}
