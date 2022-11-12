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
    pub fn none() -> Self {
        Self {
            t: 0.0,
            u: 0.0,
            v: 0.0,
            point: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn is_some(&self) -> bool {
        self.t > 0.0
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
