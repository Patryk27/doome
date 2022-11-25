use doome_bevy::prelude::Assets;

use super::definition::WeaponDefinition;
use super::WeaponSprites;

pub fn rifle(assets: &Assets) -> (WeaponDefinition, WeaponSprites) {
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
        .with_cooldown(0.5)
        .with_speed(10.0)
        .with_damage(10.0)
        .with_limited_ammo(20);

    (definition, sprites)
}

pub fn handgun(assets: &Assets) -> (WeaponDefinition, WeaponSprites) {
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
        .with_cooldown(0.2)
        .with_speed(10.0)
        .with_damage(10.0)
        .with_limited_ammo(20);

    (definition, sprites)
}

pub fn rpg(assets: &Assets) -> (WeaponDefinition, WeaponSprites) {
    let idle = assets.load_image("rpg");
    let animation = vec![assets.load_image("rpg")];

    let ui_icon = assets.load_image("rpg_outline");

    let sprites = WeaponSprites {
        idle,
        animation,
        ui_icon,
    };

    let definition = WeaponDefinition::new()
        .with_cooldown(1.0)
        .with_speed(10.0)
        .with_damage(10.0)
        .with_limited_ammo(1);

    (definition, sprites)
}
