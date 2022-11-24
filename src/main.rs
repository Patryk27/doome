#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![feature(map_first_last)]

mod camera;
mod charon;
mod commands;
mod explosions;
mod interaction;
mod levels;
mod markers;
mod objects;
mod player;
mod ui;
mod units;

mod prelude {
    pub use bevy::prelude::*;
    pub use doome_bevy::prelude::*;

    pub use crate::levels::*;
    pub use crate::objects::*;
    pub use crate::ui::*;
    pub use crate::units::*;
}

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::assets::Assets;
use doome_bevy::text::TextEngine;
use doome_engine::{HEIGHT, WIDTH};

use self::levels::EnterLevel;
use self::objects::Flashlight;
use self::ui::InputLock;

// TODO: Right now we're including files like .gitignore or *.blend (and the pesky *.blend1)
//       ideally we'd remove them before including them in the binary. Perhaps a custom proc macro?
#[cfg(feature = "static-assets")]
const ASSETS: include_dir::Dir<'static> = include_dir::include_dir!("assets");

const WINDOW_SCALE: f32 = 4.0;

fn main() {
    #[cfg(feature = "static-assets")]
    let assets = Assets::init_static(&ASSETS).unwrap();

    #[cfg(not(feature = "static-assets"))]
    let assets = Assets::init("assets").unwrap();

    App::new()
        .add_event::<EnterLevel>()
        //
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
                ..Default::default()
            },
            ..Default::default()
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
        .add_plugin(charon::CharonPlugin)
        .add_plugin(commands::CommandsPlugin)
        // Misc systems
        .add_startup_system(hide_cursor)
        .add_startup_system(player::spawn)
        .add_startup_system(levels::start)
        .add_system(doome_bevy::simple_animations::rotate)
        .add_system(doome_bevy::simple_animations::float)
        .add_system(doome_bevy::model_animation::animate)
        .add_system(quit_on_exit)
        .add_system(player::process_movement)
        .add_system(camera::sync_with_player)
        .add_system(Flashlight::sync_with_player)
        .add_system(explosions::update_explosions)
        .add_system(camera::sync_with_player)
        .add_system(levels::level1::init)
        .add_system(levels::level1::process)
        .add_system(levels::level2::init)
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
