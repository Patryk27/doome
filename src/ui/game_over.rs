use doome_bevy::doome::DoomeRenderer;
use doome_engine::Canvas;
use image::RgbaImage;

use crate::prelude::*;

pub fn setup(mut commands: Commands, assets: Res<Assets>) {
    let you_died = assets.load_image("you_died");
    let click_to_restart = assets.load_image("click_to_restart");

    commands.insert_resource(GameOverAssets {
        you_died,
        click_to_restart,
    });

    commands.insert_resource(GameOverState {
        timer: 0.0,
        is_showing_click_to_restart: false,
    });
}

#[derive(Resource)]
pub struct GameOverAssets {
    you_died: AssetHandle<RgbaImage>,
    click_to_restart: AssetHandle<RgbaImage>,
}

const CLICK_TO_RESTART_SWITCH_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct GameOverState {
    timer: f32,
    is_showing_click_to_restart: bool,
}

pub fn render(
    time: Res<Time>,
    assets: Res<Assets>,
    game_over_assets: Res<GameOverAssets>,
    mut state: ResMut<GameOverState>,
    levels_coordinator: Res<LevelsCoordinator>,
    mut renderer: ResMut<DoomeRenderer>,
) {
    if !levels_coordinator.is_game_over {
        return;
    }

    let delta = time.delta_seconds();
    state.timer += delta;

    if state.timer > CLICK_TO_RESTART_SWITCH_TIME {
        state.timer = 0.0;
        state.is_showing_click_to_restart = !state.is_showing_click_to_restart;
    }

    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(frame);

    let you_died = assets.image(game_over_assets.you_died);
    let click_to_restart = assets.image(game_over_assets.click_to_restart);

    canvas.blit(0, 0, you_died);

    if state.is_showing_click_to_restart {
        canvas.blit(0, 0, click_to_restart);
    }
}
