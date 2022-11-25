use crate::prelude::*;

#[derive(Component)]
pub struct LevelZone {
    pub name: String,
    pub x1: f32,
    pub z1: f32,
    pub x2: f32,
    pub z2: f32,
    pub contains_player: bool,
}

impl LevelZone {
    pub fn contains(&self, obj: &Transform) -> bool {
        let obj = obj.translation;

        (self.x1..self.x2).contains(&obj.x)
            && (self.z1..self.z2).contains(&obj.z)
    }
}
