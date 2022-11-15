use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Triangle {
    // X, Y, Z - vertex 0; W - material id
    v0: Vec4,

    // X, Y, Z - vertex 1; W - unused
    v1: Vec4,

    // X, Y, Z - vertex 2; W - unused
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

    pub fn v0(&self) -> Vec3 {
        self.v0.xyz()
    }

    pub fn v1(&self) -> Vec3 {
        self.v1.xyz()
    }

    pub fn v2(&self) -> Vec3 {
        self.v2.xyz()
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

        Hit {
            t,
            uv: vec2(u, v),
            ray,
            point: ray.origin() + ray.direction() * (t - 0.01),
            normal: v0v1.cross(v0v2).normalize(),
            triangle_id: TriangleId::new(0),
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
}

#[derive(Copy, Clone, Debug)]
pub struct TriangleId(usize);

impl TriangleId {
    pub fn new(id: usize) -> Self {
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
