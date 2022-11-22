use bevy::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Health {
    pub val: f32,
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_dead);
    }
}

fn despawn_dead(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.val <= 0.0 {
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.despawn();
            }
        }
    }
}
