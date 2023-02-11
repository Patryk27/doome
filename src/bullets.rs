use std::time::Duration;

use bevy::prelude::*;
use doome_bevy::audio::Audio;
use doome_bevy::health::Health;
use doome_bevy::physics::events::Collision;
use doome_bevy::prelude::DoomeAssets;

use crate::explosions::spawn_explosion;
use crate::player::AddScreenShake;
use crate::weapons::BulletType;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageDealt>();
        app.add_system(collide_and_apply_damage);
        app.add_system(collect_garbage);
    }
}

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub bullet_type: BulletType,
    pub lifetime: Timer,
}

impl Bullet {
    pub fn new(damage: f32, bullet_type: BulletType) -> Self {
        Self {
            damage,
            bullet_type,
            lifetime: Timer::new(Duration::from_secs(10), TimerMode::Once),
        }
    }
}

pub struct DamageDealt {
    pub amount: f32,
    pub entity: Entity,
}

fn collide_and_apply_damage(
    mut commands: Commands,
    assets: Res<DoomeAssets>,
    mut audio: ResMut<Audio>,
    mut collisions: EventReader<Collision>,
    mut health: Query<&mut Health>,
    bullets: Query<(&Bullet, &Transform)>,
    mut dmg_events: EventWriter<DamageDealt>,
    mut screen_shakes: EventWriter<AddScreenShake>,
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
                screen_shakes.send(AddScreenShake(0.5));
            }

            if let Some(mut entity) = commands.get_entity(collision.entity_a) {
                entity.despawn();
            }

            if let Ok(mut health) = health.get_mut(collision.entity_b) {
                dmg_events.send(DamageDealt {
                    amount: bullet.damage,
                    entity: collision.entity_b,
                });

                health.health = (health.health - bullet.damage)
                    .clamp(0.0, health.max_health);
            }
        }
    }
}

fn collect_garbage(
    time: Res<Time>,
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (bullet_entity, mut bullet) in bullets.iter_mut() {
        bullet.lifetime.tick(time.delta());

        if bullet.lifetime.just_finished() {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn is_a_bullet(bullets: &Query<(&Bullet, &Transform)>, entity: Entity) -> bool {
    bullets.get(entity).is_ok()
}
