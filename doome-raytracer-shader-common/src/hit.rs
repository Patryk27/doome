use crate::*;

#[derive(Copy, Clone)]
pub struct Hit {
    pub t: f32,
    pub uv: Vec2,
    pub ray: Ray,
    pub point: Vec3,
    pub normal: Vec3,
    pub triangle_id: TriangleId,
    pub material_id: MaterialId,
}

impl Hit {
    const MAX_T: f32 = 1000.0;

    pub fn none() -> Self {
        Self {
            t: Self::MAX_T,
            uv: Default::default(),
            ray: Default::default(),
            point: Default::default(),
            normal: Default::default(),
            triangle_id: TriangleId::new(0),
            material_id: MaterialId::new(0),
        }
    }

    pub fn is_some(&self) -> bool {
        self.t < Self::MAX_T
    }

    pub fn is_closer_than(&self, other: Self) -> bool {
        self.t < other.t
    }
}
