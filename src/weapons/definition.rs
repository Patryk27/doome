use doome_bevy::assets::{AssetHandle, Model};
use image::RgbaImage;

use super::sprites::WeaponSprites;

#[derive(Debug, Clone)]
pub struct WeaponDefinition {
    cooldown: f32,
    bullet_model: Option<AssetHandle<Model>>,
    bullet_speed: f32,
    bullet_damage: f32,
    bullet_scale: f32,
    forward_offset: f32,
    height_offset: f32,
    collider_radius: f32,
    limited_ammo: Option<usize>,
    bullet_type: BulletType,
}

#[derive(Debug, Clone)]
pub enum BulletType {
    Bullet,
    Rocket { explosion_radius: f32 },
}

impl WeaponDefinition {
    pub fn new() -> Self {
        Self {
            cooldown: 0.5,
            forward_offset: 1.0,
            height_offset: 1.0,
            bullet_speed: 10.0,
            bullet_model: None,
            bullet_damage: 10.0,
            bullet_scale: 1.0,
            collider_radius: 0.25,
            limited_ammo: None,
            bullet_type: BulletType::Bullet,
        }
    }

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

    pub fn with_limited_ammo(mut self, ammo: usize) -> Self {
        self.limited_ammo = Some(ammo);
        self
    }

    pub fn with_rocket(mut self, explosion_radius: f32) -> Self {
        self.bullet_type = BulletType::Rocket { explosion_radius };
        self
    }
}
