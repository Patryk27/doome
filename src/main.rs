mod levels;

use std::f32::consts::PI;

use bevy::app::AppExit;
use bevy::diagnostic::{
    Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::assets::Assets;
use doome_bevy::components::*;
use doome_bevy::doome::{DoomePlugin, DoomeRenderer};
use doome_bevy::physics::{Body, PhysicsPlugin};
use doome_bevy::renderer::RendererPlugin;
use doome_bevy::text::Text;
use doome_engine::{Canvas, HEIGHT, HUD_HEIGHT, WIDTH};
use doome_surface::Color;
use glam::vec3;
use include_dir::{include_dir, Dir};

// TODO: Right now we're including files like .gitignore or *.blend (and the pesky *.blend1)
//       ideally we'd remove them before including them in the binary. Perhaps a custom proc macro?
const ASSETS: Dir<'static> = include_dir!("assets");

const WINDOW_SCALE: f32 = 4.0;

fn main() {
    let assets = Assets::init(&ASSETS).unwrap();

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
        .add_plugin(DoomePlugin)
        .add_plugin(PhysicsPlugin::default())
        // Misc systems
        .add_system(quit_on_exit)
        .add_system(process_movement)
        .add_system(process_camera)
        .add_system(render_ui)
        .add_startup_system(hide_cursor)
        .add_startup_system(levels::level1::init)
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

fn render_ui(
    mut doome_renderer: ResMut<DoomeRenderer>,
    text: Res<Text>,
    diagnostics: Res<Diagnostics>,
) {
    let frame = &mut doome_renderer.pixels.image_data;
    let mut canvas = Canvas::new(&text.text_engine, frame);

    canvas.rect(
        0,
        HEIGHT - HUD_HEIGHT,
        WIDTH,
        HEIGHT,
        Color {
            r: 0x10,
            g: 0x10,
            b: 0x10,
            a: 0x66,
        },
    );

    let fps_diagnostic =
        diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();

    if let Some(fps) = fps_diagnostic.smoothed() {
        canvas.text(
            10,
            HEIGHT - 30,
            format!("FPS: {fps:>.6}{}", fps_diagnostic.suffix),
        );
    }
}

fn process_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<(&mut Body, &mut Transform), With<Player>>,
) {
    const MOUSE_ROTATION_SENSITIVITY: f32 = 0.5;
    const PLANAR_MOVEMENT_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = 2.0;

    let (mut body, mut transform) = player.single_mut();
    let delta = time.delta_seconds();

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
        let sign = if keys.pressed(KeyCode::A) { -1.0 } else { 1.0 };
        body.velocity += transform.left() * sign;
    }

    body.velocity = body.velocity.normalize_or_zero() * PLANAR_MOVEMENT_SPEED;
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
