use bevy::prelude::*;

use crate::physics::events::Collision;

#[derive(Component)]
pub struct Bullet;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_to_stage(CoreStage::PreUpdate, listen_to_bullet_collisions);
    }
}

fn listen_to_bullet_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<Collision>,
    bullets: Query<&Bullet>,
) {
    for collision in collision_events.iter() {
        if bullets.get(collision.entity_a).is_ok() {
            if let Some(mut entity) = commands.get_entity(collision.entity_a) {
                entity.despawn();
            }
        }
    }
}
