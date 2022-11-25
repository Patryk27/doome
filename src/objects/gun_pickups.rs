use bevy::prelude::*;

use crate::commands::{Command, Item};
use crate::pickable::Pickable;
use crate::prelude::*;

pub struct RiflePickup;

pub struct RpgPickup;

impl RiflePickup {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec3,
    ) -> Entity {
        spawn(
            assets,
            commands,
            position,
            "rifle_icon",
            Command::Give { what: Item::Rifle },
        )
    }
}

impl RpgPickup {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec3,
    ) -> Entity {
        spawn(
            assets,
            commands,
            position,
            "rpg_icon",
            Command::Give {
                what: Item::RocketLauncher,
            },
        )
    }
}

fn spawn(
    assets: &Assets,
    commands: &mut Commands,
    position: Vec3,
    icon_name: &str,
    on_pickup: Command,
) -> Entity {
    let model = assets.load_model("heart");
    let texture = assets.load_texture(icon_name);

    let position = position + vec3(0.0, 1.5, 0.0);

    commands
        .spawn((
            model,
            Transform::from_translation(position),
            Pickable { on_pickup },
            Material::default()
                .with_color(Color::hex(0xffffff))
                .with_uv_transparency()
                .with_texture(texture),
            Collider::circle(1.25, 6),
            GeometryType::Dynamic,
            Billboard,
            Float {
                anchor: position,
                amplitude: 0.4,
                speed: 2.0,
            },
        ))
        .id()
}
