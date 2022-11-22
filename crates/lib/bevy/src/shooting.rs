use bevy::prelude::*;

use crate::assets::{AssetHandle, Model};
use crate::billboard::Billboard;
use crate::bullets::Bullet;
use crate::components::{GeometryType, Material};
use crate::convert::graphical_to_physical;
use crate::physics::components::{Body, BodyType, Collider};

#[derive(Debug, Clone)]
pub struct Shooter {
    // Colldown is in seconds
    cooldown: f32,
    cooldown_timer: f32,
    bullet_speed: f32,
    bullet_model: AssetHandle<Model>,
    bullet_damage: f32,
}

impl Shooter {
    pub fn new(
        cooldown: f32,
        bullet_speed: f32,
        bullet_damage: f32,
        bullet_model: AssetHandle<Model>,
    ) -> Self {
        Self {
            cooldown,
            cooldown_timer: 0.0,
            bullet_speed,
            bullet_model,
            bullet_damage,
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.cooldown_timer <= 0.0
    }

    pub fn shoot(&mut self, transform: &Transform, commands: &mut Commands) {
        self.cooldown_timer = self.cooldown;

        // TODO: Make this configurable
        let forward = transform.forward();
        let position_offset = forward * 1.0;

        let mut bullet_transform = transform.clone();
        bullet_transform.translation += position_offset;
        bullet_transform.translation += Vec3::Y * 1.0; // from the camera

        commands.spawn((
            bullet_transform,
            self.bullet_model,
            Material::default().with_uv_transparency().emissive(),
            Collider::circle(0.25), // TODO: Configurable?
            Body {
                velocity: graphical_to_physical(
                    forward.normalize() * self.bullet_speed,
                ),
                body_type: BodyType::Kinematic,
            },
            GeometryType::Dynamic,
            Bullet {
                damage: self.bullet_damage,
            },
            Billboard,
        ));
    }

    pub fn update(&mut self, delta_time: f32) {
        self.cooldown_timer -= delta_time;
    }
}
