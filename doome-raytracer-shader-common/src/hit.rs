use crate::*;

#[derive(Copy, Clone)]
pub struct Hit {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Hit {
    const MAX_T: f32 = 1000.0;

    pub fn none() -> Self {
        Self {
            t: Self::MAX_T,
            u: 0.0,
            v: 0.0,
            point: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn is_some(&self) -> bool {
        self.t < Self::MAX_T
    }

    pub fn is_closer_than(&self, other: Self) -> bool {
        self.t < other.t
    }
}
