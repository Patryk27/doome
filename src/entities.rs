use bevy::prelude::*;
use doome_bevy::billboard::Billboard;
use doome_bevy::enemies::Enemy;
use doome_bevy::health::Health;
use doome_bevy::prelude::{
    Assets, Collider, Color, GeometryType, Material, RayCast,
};
use doome_bevy::shooting::Shooter;
use doome_bevy::simple_animations::{Float, Rotate};

pub fn spawn_moth_monster(
    commands: &mut Commands,
    assets: &Assets,
    position: Vec3,
) -> Entity {
    let moth_model = assets.load_model("moth_monster");

    commands
        .spawn((
            Enemy::new(
                Shooter::default()
                    .with_cooldown(1.0)
                    .with_damage(10.0)
                    .with_speed(10.0)
                    .with_model(assets.load_model("fireball")),
            ),
            moth_model,
            Material::default().with_uv_transparency(),
            GeometryType::Dynamic,
            Transform::from_translation(position),
            Billboard,
            Health::new(100.0),
            RayCast {
                origin: Vec2::ZERO,
                direction: Vec2::NEG_Y * 20.0,
                hit: None,
            },
            Collider::circle(0.5),
        ))
        .id()
}

pub fn spawn_heart(
    commands: &mut Commands,
    assets: &Assets,
    position: Vec3,
) -> Entity {
    let heart_model = assets.load_model("heart");

    commands
        .spawn((
            heart_model,
            Transform::from_translation(position),
            Rotate {
                inverted: false,
                speed: 0.4,
            },
            Float {
                anchor: position,
                amplitude: 0.5,
                speed: 1.0,
            },
            GeometryType::Dynamic,
            Material::default()
                .with_reflectivity(1.0)
                .with_color(Color::hex(0x000000))
                .with_reflection_color(Color::hex(0xf12421)),
        ))
        .id()
}
