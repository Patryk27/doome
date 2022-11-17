#[derive(Default)]
pub struct Geometry {
    aabbs: Vec<AABB>,
}

impl Geometry {
    pub fn add_aabb(&mut self, wall: AABB) {
        self.aabbs.push(wall);
    }

    pub fn is_free_at(&self, to_x: f32, to_y: f32) -> bool {
        true
    }
}

pub struct AABB {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}
