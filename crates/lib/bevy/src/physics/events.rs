use bevy::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Collision {
    pub entity_a: Entity,
    pub entity_b: Entity,
}
