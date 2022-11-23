use bevy::prelude::*;

// pub struct EntityDied

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Health {
    pub val: f32,
    pub is_dead: bool,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self {
            val: health,
            is_dead: false,
        }
    }
}

pub struct Death(pub Entity);

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Death>();
        app.add_system(despawn_dead);
    }
}

fn despawn_dead(
    mut deaths: EventWriter<Death>,
    mut query: Query<(Entity, &mut Health)>,
) {
    for (entity, mut health) in query.iter_mut() {
        if health.is_dead {
            continue;
        }

        if health.val <= 0.0 {
            log::info!("Annoucned death of entity {:?}", entity);
            health.is_dead = true;
            deaths.send(Death(entity));
        }
    }
}
