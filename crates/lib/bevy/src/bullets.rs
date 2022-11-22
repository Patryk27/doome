use bevy::prelude::*;

use crate::physics::events::Collision;

#[derive(Component)]
pub struct Bullet;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(listen_to_bullet_collisions);
    }
}

fn listen_to_bullet_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<Collision>,
    bullets: Query<&Bullet>,
) {
    for collision in collision_events.iter() {
        if bullets.get(collision.entity_a).is_ok() {
            commands.entity(collision.entity_a).despawn();
        }

        if bullets.get(collision.entity_b).is_ok() {
            commands.entity(collision.entity_b).despawn();
        }
    }
}
