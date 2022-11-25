use bevy::prelude::*;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::health::Health;
use doome_bevy::player::Player;
use doome_bevy::text::TextEngine;
use doome_engine::{Canvas, HEIGHT};

pub fn render(
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
    player: Query<(&Player, &Health)>,
) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new_text(&text_engine, frame);

    let (_, player_health) = player.single();

    const HEALTH_WIDTH: u16 = 100;
    const HEALTH_HEIGHT: u16 = 12;
    const HEALTH_X: u16 = 200;
    const HEALTH_Y: u16 = HEIGHT - 9 - HEALTH_HEIGHT;

    let actual_width =
        (player_health.health * HEALTH_WIDTH as f32 / 100.0).floor() as u16;

    canvas.rect(
        HEALTH_X,
        HEALTH_Y,
        HEALTH_X + actual_width,
        HEALTH_Y + HEALTH_HEIGHT - 1,
        doome_surface::Color::RED,
    );
}
