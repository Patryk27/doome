use bevy::prelude::*;

use crate::assets::{AssetHandle, Model};
use crate::billboard::Billboard;
use crate::bullets::Bullet;
use crate::components::{GeometryType, Material};
use crate::convert::graphical_to_physical;
use crate::physics::components::{Body, BodyType, Collider};

#[derive(Debug, Clone)]
pub struct Shooter {
    // Cooldown is in seconds
    cooldown: f32,
    cooldown_timer: f32,
    bullet_speed: f32,
    bullet_model: Option<AssetHandle<Model>>,
    bullet_damage: f32,
    bullet_scale: f32,
    forward_offset: f32,
    height_offset: f32,
    collider_radius: f32,
}

impl Shooter {
    pub fn with_model(mut self, model: AssetHandle<Model>) -> Self {
        self.bullet_model = Some(model);
        self
    }

    pub fn with_cooldown(mut self, cooldown: f32) -> Self {
        self.cooldown = cooldown;
        self
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.bullet_speed = speed;
        self
    }

    pub fn with_damage(mut self, damage: f32) -> Self {
        self.bullet_damage = damage;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.bullet_scale = scale;
        self
    }

    pub fn with_forward_offset(mut self, offset: f32) -> Self {
        self.forward_offset = offset;
        self
    }

    pub fn with_height_offset(mut self, offset: f32) -> Self {
        self.height_offset = offset;
        self
    }

    pub fn with_collider_radius(mut self, radius: f32) -> Self {
        self.collider_radius = radius;
        self
    }

    pub fn can_shoot(&self) -> bool {
        self.cooldown_timer <= 0.0
    }

    pub fn shoot(&mut self, transform: &Transform, commands: &mut Commands) {
        self.cooldown_timer = self.cooldown;

        let forward = transform.forward();
        let position_offset = forward * self.forward_offset;

        let mut bullet_transform = transform.clone();
        bullet_transform.translation += position_offset;
        bullet_transform.translation += Vec3::Y * self.height_offset; // from the camera
        bullet_transform.scale = Vec3::ONE * self.bullet_scale;

        let mut cmds = commands.spawn((
            bullet_transform,
            Material::default().with_uv_transparency().emissive(),
            Collider::circle(self.collider_radius, 6),
            Body {
                acceleration: Vec2::ZERO,
                velocity: graphical_to_physical(
                    forward.normalize() * self.bullet_speed,
                ),
                body_type: BodyType::Ethereal,
            },
            GeometryType::Dynamic,
            Bullet {
                damage: self.bullet_damage,
            },
            Billboard,
        ));

        if let Some(model) = self.bullet_model {
            cmds.insert(model);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.cooldown_timer -= delta_time;
    }
}

impl Default for Shooter {
    fn default() -> Self {
        Self {
            cooldown: 0.5,
            cooldown_timer: 0.0,
            forward_offset: 1.0,
            height_offset: 1.0,
            bullet_speed: 10.0,
            bullet_model: None,
            bullet_damage: 10.0,
            bullet_scale: 1.0,
            collider_radius: 0.25,
        }
    }
}
