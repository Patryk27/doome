use bevy::prelude::*;
use doome_bevy::audio::Audio;
use doome_bevy::health::Health;
use doome_bevy::physics::events::Collision;
use doome_bevy::prelude::Assets;

use crate::explosions::spawn_explosion;
use crate::weapons::BulletType;

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub bullet_type: BulletType,
}

impl Bullet {
    pub fn new(damage: f32, bullet_type: BulletType) -> Self {
        Self {
            damage,
            bullet_type,
        }
    }
}

pub struct BulletsPlugin;

pub struct DamageDealt {
    pub amount: f32,
    pub entity: Entity,
}

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DamageDealt>();
        app.add_system(collide_and_apply_damage);
    }
}

fn collide_and_apply_damage(
    mut commands: Commands,
    assets: Res<Assets>,
    mut audio: ResMut<Audio>,
    mut collisions: EventReader<Collision>,
    mut health: Query<&mut Health>,
    bullets: Query<(&Bullet, &Transform)>,
    mut dmg_events: EventWriter<DamageDealt>,
) {
    for collision in collisions.iter() {
        if let Ok((bullet, transform)) = bullets.get(collision.entity_a) {
            if is_a_bullet(&bullets, collision.entity_b) {
                continue;
            }

            if let BulletType::Rocket { explosion_radius } = bullet.bullet_type
            {
                spawn_explosion(
                    &mut commands,
                    &assets,
                    &mut audio,
                    transform.clone().with_scale(Vec3::ONE * explosion_radius),
                );
            }

            if let Some(mut entity) = commands.get_entity(collision.entity_a) {
                entity.despawn();
            }

            if let Ok(mut health) = health.get_mut(collision.entity_b) {
                dmg_events.send(DamageDealt {
                    amount: bullet.damage,
                    entity: collision.entity_b,
                });
                health.health -= bullet.damage;
            }
        }
    }
}

fn is_a_bullet(bullets: &Query<(&Bullet, &Transform)>, entity: Entity) -> bool {
    bullets.get(entity).is_ok()
}
