use bevy::prelude::*;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::prelude::*;
use doome_engine::Canvas;
use image::RgbaImage;

use super::{HEALTHY_THRESHOLD, WOUNDED_THRESHOLD};

const PULSE_FREQUENCY: f32 = 10.0;

pub fn setup(mut commands: Commands, assets: Res<Assets>) {
    let blood_image = assets.load_image("blood_overlay");

    commands.insert_resource(BloodState { blood_image });
}

#[derive(Resource)]
pub struct BloodState {
    blood_image: AssetHandle<RgbaImage>,
}

pub fn render(
    assets: Res<Assets>,
    state: Res<BloodState>,
    time: Res<Time>,
    mut renderer: ResMut<DoomeRenderer>,
    player: Query<(&Player, &Health)>,
) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(frame);

    let (_player, health) = player.single();

    let health = health.health;

    if health > HEALTHY_THRESHOLD {
        return;
    }

    let blood = assets.image(state.blood_image);

    let pulse = (PULSE_FREQUENCY * time.elapsed_seconds()).sin() * 0.5 + 0.5;

    if health > WOUNDED_THRESHOLD {
        let a = crate::math::remap(pulse, 0.0, 1.0, 100.0, 120.0) as u8;
        canvas.blit_blended(
            0,
            0,
            blood,
            doome_surface::Color {
                r: 255,
                g: 255,
                b: 255,
                a,
            },
        );
    } else {
        let a = crate::math::remap(pulse, 0.0, 1.0, 160.0, 200.0) as u8;

        let fill_clor = doome_surface::Color::RED.with_a(4);

        canvas.fill(fill_clor);

        canvas.blit_blended(
            0,
            0,
            blood,
            doome_surface::Color {
                r: 255,
                g: 255,
                b: 255,
                a,
            },
        );
    }
}
