use std::sync::Arc;

use doome_bevy::prelude::Assets;

use super::definition::WeaponDefinition;
use super::WeaponSprites;

pub fn rifle(assets: &Assets) -> (Arc<WeaponDefinition>, Arc<WeaponSprites>) {
    let idle = assets.load_image("rifle_0");
    let animation =
        vec![assets.load_image("rifle_1"), assets.load_image("rifle_2")];

    let ui_icon = assets.load_image("rifle_outline");

    let sprites = WeaponSprites {
        idle,
        animation,
        ui_icon,
    };

    let definition = WeaponDefinition::new()
        .with_cooldown(0.3)
        .with_speed(50.0)
        .with_damage(35.0)
        .with_limited_ammo(40);

    (Arc::new(definition), Arc::new(sprites))
}

pub fn handgun(assets: &Assets) -> (Arc<WeaponDefinition>, Arc<WeaponSprites>) {
    let idle = assets.load_image("gun_1");
    let animation = vec![
        assets.load_image("gun_shoot_1"),
        assets.load_image("gun_shoot_2"),
        assets.load_image("gun_shoot_3"),
    ];

    let ui_icon = assets.load_image("handgun_outline");

    let sprites = WeaponSprites {
        idle,
        animation,
        ui_icon,
    };

    let definition = WeaponDefinition::new()
        .with_cooldown(0.6)
        .with_speed(50.0)
        .with_damage(35.0)
        .with_limited_ammo(20);

    (Arc::new(definition), Arc::new(sprites))
}

pub fn rpg(assets: &Assets) -> (Arc<WeaponDefinition>, Arc<WeaponSprites>) {
    let idle = assets.load_image("rpg");
    let animation = vec![assets.load_image("rpg")];

    let ui_icon = assets.load_image("rpg_outline");

    let sprites = WeaponSprites {
        idle,
        animation,
        ui_icon,
    };

    let definition = WeaponDefinition::new()
        .with_model(assets.load_model("fireball"))
        .with_rocket(6.0)
        .with_cooldown(0.5)
        .with_speed(20.0)
        .with_damage(500.0)
        .with_limited_ammo(1);

    (Arc::new(definition), Arc::new(sprites))
}

pub fn enemy_fire_spew(assets: &Assets) -> WeaponDefinition {
    WeaponDefinition::new()
        .with_model(assets.load_model("fireball"))
        .with_cooldown(1.0)
        .with_speed(10.0)
        .with_damage(10.0)
        .with_collider_radius(0.5)
}
