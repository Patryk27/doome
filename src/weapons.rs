use std::sync::Arc;

use bevy::prelude::*;
use doome_bevy::convert::graphical_to_physical;

use crate::bullets::Bullet;
use crate::prelude::*;

mod definition;
pub mod definitions;
mod sprites;

pub use definition::*;
pub use sprites::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        let assets = app.world.resource::<Assets>();

        app.insert_resource(PrefabWeapons {
            rifle: definitions::rifle(&assets),
            handgun: definitions::handgun(&assets),
            rpg: definitions::rpg(&assets),
        });

        app.add_system(update_cooldown);
    }
}

#[derive(Resource)]
pub struct PrefabWeapons {
    pub handgun: (Arc<WeaponDefinition>, Arc<WeaponSprites>),
    pub rifle: (Arc<WeaponDefinition>, Arc<WeaponSprites>),
    pub rpg: (Arc<WeaponDefinition>, Arc<WeaponSprites>),
}

#[derive(Component)]
pub struct Weapon {
    pub definition: Arc<WeaponDefinition>,
    pub cooldown_timer: f32,
    // None if unlimited
    pub ammo: Option<usize>,
}

impl Weapon {
    pub fn new(definition: Arc<WeaponDefinition>) -> Self {
        Self {
            ammo: definition.limited_ammo,
            definition,
            cooldown_timer: 0.0,
        }
    }

    pub fn update_def(&mut self, definition: Arc<WeaponDefinition>) {
        self.cooldown_timer = definition.cooldown;
        self.definition = definition;
        self.ammo = self.definition.limited_ammo;
    }

    pub fn can_shoot(&self) -> bool {
        self.cooldown_timer <= 0.0
    }

    pub fn out_of_ammo(&self) -> bool {
        self.ammo.map(|ammo| ammo == 0).unwrap_or(false)
    }

    pub fn shoot(&mut self, commands: &mut Commands, transform: &Transform) {
        self.cooldown_timer = self.definition.cooldown;

        if let Some(ammo) = self.ammo.as_mut() {
            *ammo = ammo.saturating_sub(1);
        }

        self.cooldown_timer = self.definition.cooldown;

        let forward = transform.forward();
        let position_offset = forward * self.definition.forward_offset;

        let mut bullet_transform = transform.clone();
        bullet_transform.translation += position_offset;
        bullet_transform.translation += Vec3::Y * self.definition.height_offset; // from the camera
        bullet_transform.scale = Vec3::ONE * self.definition.bullet_scale;

        let mut cmds = commands.spawn((
            bullet_transform,
            Material::default().with_uv_transparency().emissive(),
            Collider::circle(self.definition.collider_radius, 6),
            Body {
                acceleration: Vec2::ZERO,
                velocity: graphical_to_physical(
                    forward.normalize() * self.definition.bullet_speed,
                ),
                body_type: BodyType::Ethereal,
            },
            GeometryType::Dynamic,
            Bullet::new(
                self.definition.bullet_damage,
                self.definition.bullet_type,
            ),
            Billboard,
        ));

        if let Some(model) = self.definition.bullet_model {
            cmds.insert(model);
        }
    }
}

fn update_cooldown(time: Res<Time>, mut weapons: Query<&mut Weapon>) {
    for mut weapon in weapons.iter_mut() {
        weapon.cooldown_timer = (weapon.cooldown_timer - time.delta_seconds())
            .clamp(0.0, weapon.definition.cooldown);
    }
}
