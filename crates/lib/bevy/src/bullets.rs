use bevy::prelude::*;

use crate::physics::events::Collision;

#[derive(Component)]
pub struct Bullet {
    speed: f32,
}

impl Bullet {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
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
