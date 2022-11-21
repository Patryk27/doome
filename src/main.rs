mod billboard;
mod enemies;
mod interaction;
mod levels;
mod markers;
mod player;
mod ui_and_2d;

use std::sync::{Arc, Mutex};

use bevy::app::AppExit;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::assets::Assets;
use doome_bevy::audio::AudioPlugin;
use doome_bevy::components::*;
use doome_bevy::doome::DoomePlugin;
use doome_bevy::physics::{Body, PhysicsPlugin};
use doome_bevy::renderer::RendererPlugin;
use doome_bevy::text::Text;
use doome_engine::{HEIGHT, WIDTH};
use glam::vec3;
use ui_and_2d::UiAnd2dPlugin;

use self::player::*;

// TODO: Right now we're including files like .gitignore or *.blend (and the pesky *.blend1)
//       ideally we'd remove them before including them in the binary. Perhaps a custom proc macro?
#[cfg(feature = "static-assets")]
const ASSETS: include_dir::Dir<'static> = include_dir::include_dir!("assets");

#[cfg(not(target_arch = "wasm32"))]
const WINDOW_SCALE: f32 = 4.0;

// TODO
#[cfg(target_arch = "wasm32")]
const WINDOW_SCALE: f32 = 3.5;

fn main() {
    #[cfg(feature = "static-assets")]
    let assets = Assets::init_static(&ASSETS).unwrap();

    #[cfg(not(feature = "static-assets"))]
    let assets = Assets::init("assets").unwrap();

    let shader =
        Arc::new(Mutex::new(Some(doome_raytracer::Raytracer::shader())));

    App::new()
        .insert_resource(assets)
        .insert_resource(Text::default())
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
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(bevy::winit::WinitPlugin::default())
        // Internal plugins
        .add_plugin(RendererPlugin)
        .add_plugin(DoomePlugin { shader })
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(UiAnd2dPlugin)
        .add_plugin(AudioPlugin)
        // Game plugins
        .add_plugin(enemies::EnemiesPlugin)
        .add_plugin(billboard::BillboardPlugin)
        // Misc systems
        .add_system(quit_on_exit)
        .add_system(process_movement)
        .add_system(process_camera)
        .add_startup_system(hide_cursor)
        .add_startup_system(levels::level1::init)
        .add_system(levels::level1::process)
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

    // TODO a bit wonky
    #[cfg(not(target_arch = "wasm32"))]
    for ev in mouse_motion.iter() {
        transform.rotate_axis(
            Vec3::Y,
            MOUSE_ROTATION_SENSITIVITY * ev.delta.x * delta,
        );
    }

    body.velocity = Vec3::ZERO;

    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::S) {
        let sign = if keys.pressed(KeyCode::W) { 1.0 } else { -1.0 };
        body.velocity += transform.forward() * sign;
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
        body.velocity += transform.left() * sign;
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

    camera.origin = vec3(pos.x, 1.0, pos.z);
    camera.look_at = camera.origin + transform.forward() * 5.0;
}
