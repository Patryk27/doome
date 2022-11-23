use bevy::prelude::*;
use doome_bevy::billboard::Billboard;
use doome_bevy::enemies::Enemy;
use doome_bevy::health::Health;
use doome_bevy::prelude::{Assets, Collider, GeometryType, Material, RayCast};
use doome_bevy::shooting::Shooter;

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
