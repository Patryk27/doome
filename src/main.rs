#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod charon;
mod commands;
mod entities;
mod explosions;
mod interaction;
mod levels;
mod markers;
mod ui;

use bevy::app::AppExit;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::assets::Assets;
use doome_bevy::components::*;
use doome_bevy::convert::graphical_to_physical;
use doome_bevy::physics::components::Body;
use doome_bevy::player::Player;
use doome_bevy::text::TextEngine;
use doome_engine::{HEIGHT, WIDTH};
use glam::vec3;

// TODO: Right now we're including files like .gitignore or *.blend (and the pesky *.blend1)
//       ideally we'd remove them before including them in the binary. Perhaps a custom proc macro?
#[cfg(feature = "static-assets")]
const ASSETS: include_dir::Dir<'static> = include_dir::include_dir!("assets");

#[cfg(not(target_arch = "wasm32"))]
const WINDOW_SCALE: f32 = 4.0;

// TODO
#[cfg(target_arch = "wasm32")]
const WINDOW_SCALE: f32 = 3.5;

#[derive(Resource)]
pub struct InputLock {
    pub is_locked: bool,
}

fn main() {
    #[cfg(feature = "static-assets")]
    let assets = Assets::init_static(&ASSETS).unwrap();

    #[cfg(not(feature = "static-assets"))]
    let assets = Assets::init("assets").unwrap();

    App::new()
        .insert_resource(InputLock { is_locked: false })
        .insert_resource(assets)
        .insert_resource(TextEngine::default())
        // Bevy plugins
        .add_plugin(bevy::log::LogPlugin::default())
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::time::TimePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::hierarchy::HierarchyPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(bevy::window::WindowPlugin {
            window: WindowDescriptor {
                title: "Doomé".to_string(),
                width: WIDTH as f32 * WINDOW_SCALE,
                height: HEIGHT as f32 * WINDOW_SCALE,
                ..WindowDescriptor::default()
            },
            ..bevy::window::WindowPlugin::default()
        })
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::winit::WinitPlugin::default())
        // Internal plugins
        .add_plugin(doome_bevy::renderer::RendererPlugin)
        .add_plugin(doome_bevy::doome::DoomePlugin)
        .add_plugin(doome_bevy::physics::PhysicsPlugin::default())
        .add_plugin(doome_bevy::audio::AudioPlugin)
        .add_plugin(doome_bevy::enemies::EnemiesPlugin)
        .add_plugin(doome_bevy::billboard::BillboardPlugin)
        .add_plugin(doome_bevy::bullets::BulletsPlugin)
        .add_plugin(doome_bevy::health::HealthPlugin)
        // Game plugins
        .add_plugin(ui::UiPlugin)
        .add_plugin(charon::Charon)
        .add_plugin(commands::CommandsPlugin)
        // Misc systems
        .add_system(doome_bevy::simple_animations::rotate)
        .add_system(doome_bevy::simple_animations::float)
        .add_system(doome_bevy::model_animation::animate)
        .add_system(quit_on_exit)
        .add_system(process_movement)
        .add_system(process_camera)
        .add_system(explosions::update_explosions)
        .add_startup_system(hide_cursor)
        // .add_startup_system(levels::level1::init)
        // .add_system(levels::level1::process)
        .add_startup_system(levels::level2::init)
        .add_system(levels::level2::process)
        .run();
}

fn hide_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_grab_mode(CursorGrabMode::Confined);
    window.set_cursor_visibility(false);
}

fn quit_on_exit(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn process_movement(
    input_lock: Res<InputLock>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<(&Player, &mut Body, &mut Transform)>,
) {
    const MOUSE_ROTATION_SENSITIVITY: f32 = 0.5;
    const PLANAR_MOVEMENT_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = 2.0;

    let (player, mut body, mut transform) = player.single_mut();
    let delta = time.delta_seconds();

    if input_lock.is_locked {
        return;
    }

    // TODO a bit wonky
    #[cfg(not(target_arch = "wasm32"))]
    for ev in mouse_motion.iter() {
        transform.rotate_axis(
            Vec3::Y,
            MOUSE_ROTATION_SENSITIVITY * ev.delta.x * delta,
        );
    }

    body.velocity = Vec2::ZERO;

    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::S) {
        let sign = if keys.pressed(KeyCode::W) { 1.0 } else { -1.0 };
        body.velocity += graphical_to_physical(transform.forward() * sign);
    }

    if keys.pressed(KeyCode::Comma) || keys.pressed(KeyCode::Period) {
        let sign = if keys.pressed(KeyCode::Comma) {
            -1.0
        } else {
            1.0
        };

        transform.rotate_axis(Vec3::Y, ROTATION_SPEED * sign * delta);
    }

    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::D) {
        let sign = if keys.pressed(KeyCode::A) { 1.0 } else { -1.0 };
        body.velocity += graphical_to_physical(transform.left() * sign);
    }

    body.velocity = body.velocity.normalize_or_zero() * PLANAR_MOVEMENT_SPEED;

    if !player.can_move {
        body.velocity = Default::default();
    }
}

fn process_camera(
    player: Query<(&Transform,), With<Player>>,
    mut camera: Query<&mut Camera>,
) {
    let Ok(mut camera) = camera.get_single_mut() else { return };
    let (transform,) = player.single();
    let pos = transform.translation;

    camera.origin = vec3(pos.x, 1.2, pos.z);
    camera.look_at = camera.origin + transform.forward() * 5.0;
}
