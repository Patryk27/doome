use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Object {
    v0: Vec4,
    v1: Vec4,
    v2: Vec4,
    color: Vec4,
}

impl Object {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, color: Vec3) -> Self {
        Self {
            v0: v0.extend(0.0),
            v1: v1.extend(0.0),
            v2: v2.extend(0.0),
            color: color.extend(1.0),
        }
    }

    pub fn color(&self) -> Vec4 {
        self.color
    }

    pub fn hit(&self, ray: Ray) -> Hit {
        // Following the Möller-Trumbore algorithm

        let v0v1 = (self.v1 - self.v0).truncate();
        let v0v2 = (self.v2 - self.v0).truncate();
        let pvec = ray.direction.cross(v0v2);
        let det = v0v1.dot(pvec);

        if det.abs() < f32::EPSILON {
            return Hit::none();
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin - self.v0.truncate();
        let u = tvec.dot(pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return Hit::none();
        }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction.dot(qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return Hit::none();
        }

        let t = v0v2.dot(qvec) * inv_det;

        Hit { t, u, v }
    }
}
