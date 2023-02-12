#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![feature(drain_filter)]
#![feature(map_first_last)]

use bevy::prelude::*;

#[macro_use]
mod utils;

mod args;
// mod bullets;
// mod charon;
// mod commands;
// mod enemies;
// mod explosions;
mod interaction;
// mod inventory;
// mod levels;
mod markers;
mod math;
mod music;
// mod objects;
// mod pickable;
// mod player;
mod rng;
mod settings;
// mod sounds;
// mod ui;
// mod units;
// mod weapons;

mod prelude {
    pub use bevy::prelude::*;
    pub use doome_bevy::prelude::*;

    // pub use crate::bullets::*;
    // pub use crate::commands::*;
    // pub use crate::enemies::*;
    // pub use crate::inventory::*;
    // pub use crate::levels::*;
    // pub use crate::objects::*;
    pub use crate::rng::*;
    pub use crate::settings::*;
    // pub use crate::ui::*;
    // pub use crate::units::*;
    // pub use crate::weapons::*;
}

fn main() {
    App::new()
        // ==== //
        // bevy //
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .run();
}
