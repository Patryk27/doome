use std::time::Duration;

use bevy::prelude::*;
use doome_bevy::assets::{AssetHandle, Assets};
use doome_bevy::audio::Audio;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::text::Text;
use doome_engine::Canvas;
use image::RgbaImage;

use crate::player::Player;

pub struct UiAnd2dPlugin;

impl Plugin for UiAnd2dPlugin {
    fn build(&self, app: &mut App) {
        let assets = app.world.resource::<Assets>();

        let gun_idle = assets.load_image("gun_1");

        let gun_fire_sequence = vec![
            assets.load_image("gun_shoot_1"),
            assets.load_image("gun_shoot_2"),
            assets.load_image("gun_shoot_3"),
        ];

        app.insert_resource(Data {
            gun_idle,
            gun_fire_sequence,
        });
        app.insert_resource(TextAnimation::None);
        app.add_event::<Print>();
        app.add_startup_system(setup);
        app.add_system(trigger_shoot);
        app.add_system(update_animation);
        app.add_system(update_text);
        app.add_system(render);
    }
}

#[derive(Default)]
pub struct Print {
    text: String,
}

impl Print {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

#[derive(Resource)]
struct Data {
    gun_idle: AssetHandle<RgbaImage>,
    gun_fire_sequence: Vec<AssetHandle<RgbaImage>>,
}

#[derive(Resource)]
enum TextAnimation {
    None,

    Some {
        text: String,
        tt: f32,
        completed_at: Option<f32>,
    },
}

#[derive(Component)]
struct ShootingAnimation {
    is_firing: bool,
    current_frame: usize,
    timer: Timer,
}

fn setup(mut commands: Commands) {
    let mut timer =
        Timer::new(Duration::from_millis(100), TimerMode::Repeating);

    timer.pause();

    commands.spawn(ShootingAnimation {
        is_firing: false,
        current_frame: 0,
        timer,
    });
}

// TODO: Attach an event writer
fn trigger_shoot(
    assets: Res<Assets>,
    player: Query<&Player>,
    mut shooting_animation: Query<&mut ShootingAnimation>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut audio: ResMut<Audio>,
) {
    if !player.single().can_move {
        return;
    }

    let mut shooting_animation = shooting_animation.single_mut();

    if mouse.just_pressed(MouseButton::Left)
        || keyboard.just_pressed(KeyCode::Space)
    {
        audio.play(assets.load_sound("gun_shoot"));
        shooting_animation.is_firing = true;
        shooting_animation.current_frame = 0;
        shooting_animation.timer.unpause();
        shooting_animation.timer.reset();
    }
}

fn update_animation(
    time: Res<Time>,
    mut shooting_animation: Query<&mut ShootingAnimation>,
) {
    let mut shooting_animation = shooting_animation.single_mut();
    shooting_animation.timer.tick(time.delta());

    if shooting_animation.is_firing && shooting_animation.timer.just_finished()
    {
        shooting_animation.current_frame += 1;

        // TODO: No magic number
        if shooting_animation.current_frame == 3 {
            shooting_animation.current_frame = 0;
            shooting_animation.is_firing = false;
            shooting_animation.timer.pause();
        } else {
            shooting_animation.timer.reset();
        }
    }
}

fn update_text(
    time: Res<Time>,
    mut anim: ResMut<TextAnimation>,
    mut print: EventReader<Print>,
) {
    if let TextAnimation::Some {
        tt, completed_at, ..
    } = &mut *anim
    {
        *tt += time.delta_seconds();

        if let Some(completed_at) = completed_at {
            if *tt > *completed_at + 2.0 {
                *anim = TextAnimation::None;
            }
        }
    }

    for print in print.iter() {
        *anim = TextAnimation::Some {
            text: print.text.clone(),
            tt: 0.0,
            completed_at: None,
        };
    }
}

fn render(
    time: Res<Time>,
    assets: Res<Assets>,
    data: Res<Data>,
    shooting_anim: Query<&ShootingAnimation>,
    mut text_anim: ResMut<TextAnimation>,
    player: Query<&Player>,
    mut doome_renderer: ResMut<DoomeRenderer>,
    text: Res<Text>,
) {
    let frame = &mut doome_renderer.pixels.image_data;
    let mut canvas = Canvas::new(&text.text_engine, frame);

    canvas.clear();

    // -----

    if let TextAnimation::Some {
        text,
        tt,
        completed_at,
    } = &mut *text_anim
    {
        const BASE_FUEL_PER_CHARACTER: f32 = 0.09;

        let x = 5;
        let mut y = 5;
        let mut fuel = *tt;
        let mut ran_out_of_fuel = false;

        for line in text.lines() {
            fuel -= 2.0 * BASE_FUEL_PER_CHARACTER;

            if fuel < 0.0 {
                ran_out_of_fuel = true;
                break;
            }

            let mut actual_line = String::new();

            for ch in line.bytes() {
                let ch_required_fuel = {
                    let mut f = BASE_FUEL_PER_CHARACTER;

                    if ch % 2 == 0 {
                        f += BASE_FUEL_PER_CHARACTER / 2.0;
                    }

                    f
                };

                let ch = ch as char;

                if fuel < ch_required_fuel {
                    ran_out_of_fuel = true;
                    break;
                }

                actual_line.push(ch);
                fuel -= ch_required_fuel;
            }

            canvas.text(x, y, actual_line);

            y += 12;

            if ran_out_of_fuel {
                break;
            }
        }

        if !ran_out_of_fuel {
            if completed_at.is_none() {
                *completed_at = Some(*tt);
            }
        }
    }

    // -----

    let (sway_x, sway_y) = if player.single().can_move {
        calc_sway(time.elapsed_seconds())
    } else {
        calc_sway(0.0)
    };

    let shooting_anim = shooting_anim.single();

    let gun_image = if shooting_anim.is_firing {
        data.gun_fire_sequence[shooting_anim.current_frame]
    } else {
        data.gun_idle
    };

    canvas.blit(sway_x, sway_y, assets.image(gun_image));
}

fn calc_sway(t: f32) -> (u16, u16) {
    const MAX_SWAY: f32 = 8.0;

    let y = MAX_SWAY - (t * 1.2).cos() * MAX_SWAY;

    (0 as _, y as _)
}
