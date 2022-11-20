use std::time::Duration;

use bevy::prelude::*;
use doome_bevy::assets::{AssetHandle, Assets};
use doome_bevy::audio::Audio;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::text::Text;
use doome_engine::Canvas;
use image::RgbaImage;

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
        app.add_startup_system(setup);
        app.add_system(render_ui_and_2d);
        app.add_system(trigger_shoot);
        app.add_system(update_animation);
    }
}

#[derive(Resource)]
struct Data {
    gun_idle: AssetHandle<RgbaImage>,
    gun_fire_sequence: Vec<AssetHandle<RgbaImage>>,
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
    mut shooting_animation: Query<&mut ShootingAnimation>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut audio: ResMut<Audio>,
) {
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

fn render_ui_and_2d(
    time: Res<Time>,
    assets: Res<Assets>,
    data: Res<Data>,
    animation: Query<&ShootingAnimation>,
    mut doome_renderer: ResMut<DoomeRenderer>,
    text: Res<Text>,
) {
    let frame = &mut doome_renderer.pixels.image_data;
    let mut canvas = Canvas::new(&text.text_engine, frame);

    canvas.clear();

    let (sway_x, sway_y) = calc_sway(time.elapsed_seconds());
    let anim = animation.single();

    let gun_image = if anim.is_firing {
        data.gun_fire_sequence[anim.current_frame]
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
