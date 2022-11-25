use bevy::prelude::*;

use self::definition::WeaponDefinition;
use crate::prelude::*;

mod definition;
mod sprites;
mod weapons;

pub use definition::*;
pub use sprites::*;
pub use weapons::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        let assets = app.world.resource::<Assets>();

        app.insert_resource(Weapons {
            rifle: rifle(&assets),
            handgun: handgun(&assets),
            rpg: rpg(&assets),
        });
    }
}

#[derive(Resource)]
pub struct Weapons {
    pub handgun: (WeaponDefinition, WeaponSprites),
    pub rifle: (WeaponDefinition, WeaponSprites),
    pub rpg: (WeaponDefinition, WeaponSprites),
}

#[derive(Component)]
pub struct PlayerWeapon {
    pub weapon: WeaponDefinition,
    pub weapon_sprites: WeaponSprites,
    pub cooldown: f32,
    pub ammo: i32,
}

#[derive(Component)]
pub struct EnemyWeapon {
    pub weapon: WeaponDefinition,
    pub cooldown: f32,
}

#[derive(Component)]
pub struct Weapon {
    pub definition: definition::WeaponDefinition,
    pub cooldown_timer: f32,
    // None if unlimited
    pub ammo: Option<usize>,
}
