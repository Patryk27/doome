use crate::prelude::*;

pub struct MothMonster;

impl MothMonster {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec3,
    ) -> Entity {
        let model = assets.load_model("moth_monster");

        commands
            .spawn((
                Enemy::new(
                    Shooter::default()
                        .with_cooldown(1.0)
                        .with_damage(10.0)
                        .with_speed(10.0)
                        .with_model(assets.load_model("fireball")),
                ),
                model,
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
                Collider::circle(0.5, 6),
            ))
            .id()
    }
}
