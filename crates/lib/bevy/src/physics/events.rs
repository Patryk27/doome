use bevy::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Collision {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl Collision {
    pub fn collides_with(self, other: Entity) -> bool {
        self.entity_a == other || self.entity_b == other
    }
}
