#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![feature(drain_filter)]
#![feature(map_first_last)]

use bevy::prelude::*;
use glam::vec3;

#[macro_use]
mod utils;

mod args;
// mod bullets;
// mod charon;
mod commands;
// mod enemies;
// mod explosions;
mod interaction;
mod inventory;
mod levels;
mod markers;
mod math;
mod music;
mod objects;
mod pickable;
// mod player;
mod rng;
mod settings;
// mod sounds;
// mod ui;
// mod units;
// mod weapons;
mod text;

mod prelude {
    pub use bevy::prelude::*;
    pub use doome_bevy::prelude::*;

    // pub use crate::bullets::*;
    pub use crate::commands::*;
    // pub use crate::ui::*;
    // pub use crate::units::*;
    // pub use crate::weapons::*;
    // pub use crate::enemies::*;
    pub use crate::inventory::*;
    pub use crate::levels::*;
    pub use crate::objects::*;
    pub use crate::rng::*;
    pub use crate::settings::*;
    pub use crate::text::*;
}

fn main() {
    App::new()
        // ==== //
        // bevy //
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let quad = meshes.add(Mesh::from(shape::Quad::default()));
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..default()
    });

    let positions = vec![
        vec3(0.0, 0.0, 0.0),
        vec3(1.2, 0.0, 0.0),
        vec3(-1.2, 0.0, 0.0),
    ];

    for position in &positions {
        commands.spawn(PbrBundle {
            mesh: quad.clone(),
            material: material.clone(),
            transform: Transform::from_translation(*position),
            ..default()
        });
    }
}
