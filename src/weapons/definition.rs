use doome_bevy::assets::{AssetHandle, Model};

#[derive(Debug, Clone)]
pub struct WeaponDefinition {
    pub cooldown: f32,
    pub bullet_model: Option<AssetHandle<Model>>,
    pub bullet_speed: f32,
    pub bullet_damage: f32,
    pub bullet_scale: f32,
    pub forward_offset: f32,
    pub height_offset: f32,
    pub collider_radius: f32,
    pub limited_ammo: Option<usize>,
    pub bullet_type: BulletType,
}

#[derive(Debug, Clone, Copy)]
pub enum BulletType {
    Bullet,
    Rocket { explosion_radius: f32 },
}

impl WeaponDefinition {
    pub fn new() -> Self {
        Self {
            cooldown: 0.5,
            forward_offset: 1.5,
            height_offset: 1.0,
            bullet_speed: 10.0,
            bullet_model: None,
            bullet_damage: 10.0,
            bullet_scale: 1.0,
            collider_radius: 0.5,
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
