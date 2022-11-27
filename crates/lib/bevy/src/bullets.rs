use bevy::prelude::*;

use crate::health::Health;
use crate::physics::events::Collision;

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
}

impl Bullet {
    pub fn new(damage: f32) -> Self {
        Self { damage }
    }
}

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(collide_and_apply_damage);
    }
}

fn collide_and_apply_damage(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    mut health: Query<&mut Health>,
    bullets: Query<&Bullet>,
) {
    for collision in collisions.iter() {
        if let Ok(bullet) = bullets.get(collision.entity_a) {
            if is_a_bullet(&bullets, collision.entity_b) {
                continue;
            }

            if let Some(mut entity) = commands.get_entity(collision.entity_a) {
                entity.despawn();
            }

            if let Ok(mut health) = health.get_mut(collision.entity_b) {
                health.val -= bullet.damage;
            }
        }
    }
}

fn is_a_bullet(bullets: &Query<&Bullet>, entity: Entity) -> bool {
    bullets.get(entity).is_ok()
}