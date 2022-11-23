mod text;
mod typewriter;

use std::time::Duration;

use bevy::prelude::*;
use doome_bevy::assets::{AssetHandle, Assets};
use doome_bevy::audio::Audio;
use doome_bevy::billboard::Billboard;
use doome_bevy::bullets::Bullet;
use doome_bevy::components::*;
use doome_bevy::convert::graphical_to_physical;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::health::Health;
use doome_bevy::physics::components::{Body, BodyType, Collider};
use doome_bevy::player::Player;
use doome_bevy::text::TextEngine;
use doome_engine::{Canvas, HEIGHT, WIDTH};
use glam::{vec2, Vec3Swizzles};
use image::RgbaImage;

pub use self::text::*;
pub use self::typewriter::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let assets = app.world.resource::<Assets>();
        let gun_idle = assets.load_image("gun_1");

        let gun_fire_sequence = vec![
            assets.load_image("gun_shoot_1"),
            assets.load_image("gun_shoot_2"),
            assets.load_image("gun_shoot_3"),
        ];

        // Typewriter
        app.insert_resource(Typewriter::Idle)
            .add_event::<TypewriterPrint>()
            .add_system(typewriter::update);

        // Gun
        app.insert_resource(Gun {
            gun_idle,
            gun_fire_sequence,
        })
        .add_system(trigger_shoot)
        .add_system(update_gun);

        // UI
        app.add_startup_system(setup).add_system(render);
    }
}

#[derive(Resource)]
struct Gun {
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
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<Assets>,
    mut player: Query<(&mut Player, &Transform)>,
    mut shooting_animation: Query<&mut ShootingAnimation>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut audio: ResMut<Audio>,
) {
    let (mut player, player_transform) = player.single_mut();

    if !player.can_move {
        return;
    }

    let delta = time.delta_seconds();

    player.shooter.update(delta);

    if !player.shooter.can_shoot() {
        return;
    }

    let mut shooting_animation = shooting_animation.single_mut();

    if mouse.pressed(MouseButton::Left) || keyboard.pressed(KeyCode::Space) {
        audio.play(assets.load_sound("gun_shoot"));
        shooting_animation.is_firing = true;
        shooting_animation.current_frame = 0;
        shooting_animation.timer.unpause();
        shooting_animation.timer.reset();

        player.shooter.shoot(player_transform, &mut commands);
    }
}

fn update_gun(
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

const GUN_OFFSET_Y: u16 = 16;

fn render(
    time: Res<Time>,
    assets: Res<Assets>,
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
    data: Res<Gun>,
    shooting_anim: Query<&ShootingAnimation>,
    typewriter: Res<Typewriter>,
    player: Query<(&Player, &Health)>,
    texts: Query<(&Text, &Transform, Option<&Visibility>)>,
) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new(&text_engine, frame);

    canvas.clear();

    let (player, player_health) = player.single();

    // --- //
    // Gun //

    let (sway_x, sway_y) = if player.can_move {
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

    canvas.blit(sway_x, sway_y - GUN_OFFSET_Y, assets.image(gun_image));

    // --- //
    // Ui  //

    let ui_image = assets.load_image("ui_static");
    let angrey_face = assets.load_image("angrey");
    canvas.blit(0, 0, assets.image(ui_image));
    canvas.blit(0, 0, assets.image(angrey_face));

    // ------ //
    // Health //

    const HEALTH_WIDTH: u16 = 100;
    const HEALTH_HEIGHT: u16 = 12;
    const HEALTH_X: u16 = 200;
    const HEALTH_Y: u16 = HEIGHT - 9 - HEALTH_HEIGHT;

    let actual_width =
        (player_health.val * HEALTH_WIDTH as f32 / 100.0).floor() as u16;

    canvas.rect(
        HEALTH_X,
        HEALTH_Y,
        HEALTH_X + actual_width,
        HEALTH_Y + HEALTH_HEIGHT - 1,
        doome_surface::Color::RED,
    );

    // ---------- //
    // Typewriter //

    typewriter.render(&mut canvas);

    // ------//
    // Texts //

    for (text, transform, visibility) in texts.iter() {
        let is_visible = visibility.map_or(true, |vis| vis.is_visible);

        if !is_visible {
            continue;
        }

        let translation =
            transform.translation.xy() * vec2(WIDTH as _, HEIGHT as _);

        canvas.text(
            translation.x as _,
            translation.y as _,
            &text.text,
            text.centered,
        );
    }
}

fn calc_sway(t: f32) -> (u16, u16) {
    const MAX_SWAY: f32 = 8.0;

    let y = MAX_SWAY - (t * 1.2).cos() * MAX_SWAY;

    (0 as _, y as _)
}
