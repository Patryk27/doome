use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Triangle {
    /// x,y,z is position, w is material id
    v0: Vec4,
    /// x,y,z is position of V1, w is uv.u of V0
    v1: Vec4,
    /// x,y,z is position of V2, w is uv.v of V0
    v2: Vec4,
    /// x,y is uv of V1, z,w is uv of V2
    uvs: Vec4,
}

impl Triangle {
    pub fn new(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        uv0: Vec2,
        uv1: Vec2,
        uv2: Vec2,
        material_id: MaterialId,
    ) -> Self {
        Self {
            v0: v0.extend(material_id.get() as f32),
            v1: v1.extend(uv0.x),
            v2: v2.extend(uv0.y),
            uvs: vec4(uv1.x, uv1.y, uv2.x, uv2.y),
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
        let mut u = tvec.dot(pvec) * inv_det;

        // HACK prevents funky artifacts that happen when camera is aligned in
        //      the same direction at the triangle
        if u > -0.0001 && u < 0.0 {
            u = f32::EPSILON;
        }

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

        // convert u, v to tex coords based on triangle vertices uvs
        let uv0 = vec2(self.v1.w, self.v2.w);
        let uv1 = vec2(self.uvs.x, self.uvs.y);
        let uv2 = vec2(self.uvs.z, self.uvs.w);

        let uv = uv0 + (uv1 - uv0) * u + (uv2 - uv0) * v;

        Hit {
            t,
            uv,
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
