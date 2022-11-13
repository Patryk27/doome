use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Triangle {
    v0: Vec4,
    v1: Vec4,
    v2: Vec4,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material_id: MaterialId) -> Self {
        Self {
            v0: v0.extend(material_id.get() as f32),
            v1: v1.extend(0.0),
            v2: v2.extend(0.0),
        }
    }

    pub fn hit(&self, ray: Ray) -> Hit {
        // Following the Möller-Trumbore algorithm

        let v0v1 = (self.v1 - self.v0).truncate();
        let v0v2 = (self.v2 - self.v0).truncate();
        let pvec = ray.direction().cross(v0v2);
        let det = v0v1.dot(pvec);

        if det < f32::EPSILON {
            return Hit::none();
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin() - self.v0.truncate();
        let u = tvec.dot(pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return Hit::none();
        }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction().dot(qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return Hit::none();
        }

        let t = v0v2.dot(qvec) * inv_det;

        if t < 0.0 {
            return Hit::none();
        }

        Hit {
            t,
            u,
            v,
            ray,
            point: ray.origin() + ray.direction() * (t - 0.01),
            normal: v0v1.cross(v0v2).normalize(),
            material_id: self.material_id(),
        }
    }

    fn material_id(&self) -> MaterialId {
        MaterialId::new(self.v0.w as _)
    }
}
