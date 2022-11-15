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

    pub fn v0(&self) -> Vec3 {
        self.v0.xyz()
    }

    pub fn v1(&self) -> Vec3 {
        self.v1.xyz()
    }

    pub fn v2(&self) -> Vec3 {
        self.v2.xyz()
    }

    pub fn uv0(&self) -> Vec2 {
        vec2(self.v1.w, self.v2.w)
    }

    pub fn uv1(&self) -> Vec2 {
        vec2(self.uvs.x, self.uvs.y)
    }

    pub fn uv2(&self) -> Vec2 {
        vec2(self.uvs.z, self.uvs.w)
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
        //      the same direction as the triangle
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
        let uv0 = self.uv0();
        let uv1 = self.uv1();
        let uv2 = self.uv2();

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

#[cfg(not(target_arch = "spirv"))]
impl Triangle {
    pub fn apply(mut self, xform: Mat4) -> Self {
        self.v0 =
            math::apply_transformation(self.v0.xyz(), xform).extend(self.v0.w);
        self.v1 =
            math::apply_transformation(self.v1.xyz(), xform).extend(self.v1.w);
        self.v2 =
            math::apply_transformation(self.v2.xyz(), xform).extend(self.v2.w);

        self
    }

    pub fn vertices(&self) -> [Vec3; 3] {
        [self.v0(), self.v1(), self.v2()]
    }

    pub fn center(&self) -> Vec3 {
        self.vertices().iter().sum::<Vec3>() / 3.0
    }

    pub fn set_uvs(&mut self, uv0: Vec2, uv1: Vec2, uv2: Vec2) {
        self.v1.w = uv0.x;
        self.v2.w = uv0.y;
        self.uvs = Vec4::new(uv1.x, uv1.y, uv2.x, uv2.y);
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Triangle")
            .field("material_id", &self.material_id())
            .field("v0", &self.v0())
            .field("v1", &self.v1())
            .field("v2", &self.v2())
            .field("uv0", &self.uv0())
            .field("uv1", &self.uv1())
            .field("uv2", &self.uv2())
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TriangleId(usize);

#[cfg(not(target_arch = "spirv"))]
impl TriangleId {
    pub(crate) fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn get(self) -> usize {
        self.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for TriangleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
