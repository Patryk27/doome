use std::sync::Arc;

use crate::prelude::*;
use crate::weapons::Weapon;

pub struct MothMonster;

impl MothMonster {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec3,
    ) -> Entity {
        let model = assets.load_model("moth_monster");
        let weapon = crate::weapons::definitions::enemy_fire_spew(&assets);

        commands
            .spawn((
                Enemy::new(),
                model,
                Material::default().with_uv_transparency(),
                GeometryType::Dynamic,
                Transform::from_translation(position),
                Billboard,
                Health::new(100.0, 100.0),
                RayCast {
                    origin: Vec2::ZERO,
                    direction: Vec2::NEG_Y * 20.0,
                    hit: None,
                },
                Collider::circle(0.75, 12),
                Body {
                    acceleration: Vec2::ZERO,
                    velocity: Vec2::ZERO,
                    body_type: BodyType::Kinematic,
                },
                Weapon::new(Arc::new(weapon)), // TODO: arc is inefficient here
            ))
            .id()
    }
}
