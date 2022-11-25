#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![feature(map_first_last)]

#[macro_use]
mod utils;

mod bullets;
mod charon;
mod commands;
mod explosions;
mod interaction;
mod inventory;
mod levels;
mod markers;
mod objects;
mod pickable;
mod player;
mod ui;
mod units;
mod weapons;

mod prelude {
    pub use bevy::prelude::*;
    pub use doome_bevy::prelude::*;

    pub use crate::levels::*;
    pub use crate::objects::*;
    pub use crate::ui::*;
    pub use crate::units::*;
}

use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::text::TextEngine;
use doome_engine::{HEIGHT, WIDTH};

use self::objects::Flashlight;

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
        // ==== //
        // bevy //
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
        // ========== //
        // doome_bevy //
        .insert_resource(assets)
        .insert_resource(TextEngine::default())
        .add_plugin(doome_bevy::renderer::RendererPlugin)
        .add_plugin(doome_bevy::doome::DoomePlugin)
        .add_plugin(doome_bevy::physics::PhysicsPlugin::default())
        .add_plugin(doome_bevy::audio::AudioPlugin)
        .add_plugin(doome_bevy::enemies::EnemiesPlugin)
        .add_plugin(doome_bevy::billboard::BillboardPlugin)
        .add_plugin(doome_bevy::health::HealthPlugin)
        .add_system(doome_bevy::simple_animations::rotate)
        .add_system(doome_bevy::simple_animations::float)
        .add_system(doome_bevy::model_animation::animate)
        // ===== //
        // doome //
        .add_plugin(bullets::BulletsPlugin)
        .add_plugin(weapons::WeaponsPlugin)
        .add_plugin(charon::CharonPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(commands::CommandsPlugin)
        .add_plugin(levels::LevelsPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(pickable::PickablePlugin)
        .add_plugin(inventory::InventoryPlugin)
        .add_system(explosions::update)
        .add_system(
            Flashlight::sync_with_player.after(player::process_movement),
        )
        .run();
}
