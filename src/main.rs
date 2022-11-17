mod levels;

use std::f32::consts::PI;

use bevy::app::AppExit;
use bevy::diagnostic::{
    Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::components::*;
use doome_bevy::doome::{DoomePlugin, DoomeRenderInit, DoomeRenderer};
use doome_bevy::renderer::RendererPlugin;
use doome_bevy::text::Text;
use doome_engine::pipeline::PipelineBuilder;
use doome_engine::{Canvas, HEIGHT, HUD_HEIGHT, WIDTH};
use doome_raytracer as rt;
use doome_surface::Color;
use glam::{vec2, vec3, Vec3Swizzles};

// TODO: Right now we're including files like .gitignore or *.blend (and the pesky *.blend1)
//       ideally we'd remove them before including them in the binary. Perhaps a custom proc macro?
const ASSETS: include_dir::Dir = include_dir::include_dir!("assets");

const WINDOW_SCALE: f32 = 4.0;

fn main() {
    let pipeline = {
        let pipeline = PipelineBuilder::new(ASSETS);

        // TODO

        pipeline.build()
    };

    let mut app = App::new();

    app.insert_resource(DoomeRenderInit { pipeline })
        .insert_resource(Text::default())
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
        .add_plugin(bevy::winit::WinitPlugin::default())
        .add_plugin(RendererPlugin)
        .add_plugin(DoomePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_system(quit_on_exit)
        .add_system(process_movement)
        .add_system(process_camera)
        .add_system(render_ui)
        .add_startup_system(hide_cursor)
        .add_startup_system(levels::level1::init);

    app.run();
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
    mut player: Query<(&mut Position, &mut Rotation), With<Player>>,
) {
    const MOUSE_ROTATION_SENSITIVITY: f32 = 0.5;
    const PLANAR_MOVEMENT_SPEED: f32 = 10.0;
    const CELESTIAL_MOVEMENT_SPEED: f32 = 8.0;
    const ROTATION_SPEED: f32 = 2.0;

    let (mut player_pos, mut player_rot) = player.single_mut();
    let delta = time.delta_seconds();

    // TODO
    //
    // for ev in mouse_motion.iter() {
    //     let dir = camera.look_at - camera.origin;

    //     let rot = Quat::from_axis_angle(
    //         camera.up.normalize(),
    //         // For some reason the up direction here is negative, hence the minus sign
    //         -MOUSE_ROTATION_SENSITIVITY * ev.delta.x * delta,
    //     );

    //     let new_dir = rot * dir;
    //     let new_look_at = camera.origin + new_dir;

    //     camera.look_at.x = new_look_at.x;
    //     camera.look_at.z = new_look_at.z;
    // }

    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::S) {
        let sign = if keys.pressed(KeyCode::W) { 1.0 } else { -1.0 };

        player_pos.x +=
            player_rot.angle.sin() * CELESTIAL_MOVEMENT_SPEED * delta * sign;

        player_pos.z +=
            player_rot.angle.cos() * CELESTIAL_MOVEMENT_SPEED * delta * sign;
    }

    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::D) {
        let sign = if keys.pressed(KeyCode::A) { -1.0 } else { 1.0 };

        player_rot.angle += ROTATION_SPEED * sign * delta;
    }

    if keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::E) {
        let sign = if keys.pressed(KeyCode::Q) { -1.0 } else { 1.0 };
        let angle = player_rot.angle + PI / 2.0;

        player_pos.x += angle.sin() * CELESTIAL_MOVEMENT_SPEED * delta * sign;
        player_pos.z += angle.cos() * CELESTIAL_MOVEMENT_SPEED * delta * sign;
    }
}

fn process_camera(
    player: Query<(&Position, &Rotation), With<Player>>,
    mut camera: Query<&mut Camera>,
) {
    let Ok(mut camera) = camera.get_single_mut() else { return };
    let (player_pos, player_rot) = player.single();

    camera.origin = vec3(player_pos.x, 1.0, player_pos.z);

    camera.look_at = camera.origin
        + vec3(player_rot.angle.sin(), 0.0, player_rot.angle.cos()) * 5.0;
}
