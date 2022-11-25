use bevy::prelude::*;

// pub struct EntityDied

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Health {
    pub health: f32,
    pub max_health: f32,
    pub is_dead: bool,
}

impl Health {
    pub fn new(health: f32, max_health: f32) -> Self {
        Self {
            health,
            max_health,
            is_dead: false,
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
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

        if health.health <= 0.0 {
            log::info!("Annoucned death of entity {:?}", entity);
            health.is_dead = true;
            deaths.send(Death(entity));
        }
    }
}
