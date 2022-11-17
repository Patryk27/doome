use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Triangle {
    // X, Y, Z - vertex 0; W - material id (as u16)
    v0: Vec4,

    // X, Y, Z - vertex 1; W - alpha channel (as f32, 0.0..=1.0)
    v1: Vec4,

    // X, Y, Z - vertex 2; W - whether UV transparency is enabled or not
    //                         (as bool, 0.0/1.0)
    v2: Vec4,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material_id: MaterialId) -> Self {
        Self {
            v0: v0.extend(material_id.get() as f32),
            v1: v1.extend(1.0),
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

    pub fn alpha(&self) -> f32 {
        self.v1.w
    }

    pub fn hit(&self, ray: Ray) -> Hit {
        // Following the Möller-Trumbore algorithm

        let v0v1 = (self.v1 - self.v0).truncate();
        let v0v2 = (self.v2 - self.v0).truncate();
        let pvec = ray.direction().cross(v0v2);
        let det = v0v1.dot(pvec);

        if det.abs() < f32::EPSILON {
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
            triangle_id: TriangleId::new_static(0).into_any(),
            material_id: self.material_id(),
        }
    }

    pub fn material_id(&self) -> MaterialId {
        MaterialId::new(self.v0.w as _)
    }

    pub fn has_uv_transparency(&self) -> bool {
        self.v2.w == 1.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Triangle {
    pub fn with_alpha(mut self, a: f32) -> Self {
        self.v1.w = a;
        self
    }

    pub fn with_uv_transparency(self) -> Self {
        self.with_uv_transparency_of(true)
    }

    pub fn with_uv_transparency_of(mut self, val: bool) -> Self {
        self.v2.w = if val { 1.0 } else { 0.0 };
        self
    }

    pub fn with_transform(mut self, xform: Mat4) -> Self {
        self.v0 = math::transform(self.v0.xyz(), xform).extend(self.v0.w);
        self.v1 = math::transform(self.v1.xyz(), xform).extend(self.v1.w);
        self.v2 = math::transform(self.v2.xyz(), xform).extend(self.v2.w);
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
pub struct TriangleId<T>(T, usize);

impl<T> TriangleId<T> {
    pub fn new(provenance: T, id: usize) -> Self {
        Self(provenance, id)
    }

    pub fn get(self) -> (T, usize) {
        (self.0, self.1)
    }
}

impl TriangleId<StaticTriangle> {
    pub fn new_static(id: usize) -> Self {
        Self(StaticTriangle, id)
    }

    pub fn id(self) -> usize {
        self.1
    }

    pub fn into_any(self) -> TriangleId<AnyTriangle> {
        TriangleId::new(AnyTriangle::Static, self.1)
    }
}

impl TriangleId<DynamicTriangle> {
    pub fn new_dynamic(id: usize) -> Self {
        Self(DynamicTriangle, id)
    }

    pub fn id(self) -> usize {
        self.1
    }

    pub fn into_any(self) -> TriangleId<AnyTriangle> {
        TriangleId::new(AnyTriangle::Dynamic, self.1)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for TriangleId<StaticTriangle> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct StaticTriangle;

#[derive(Copy, Clone, Debug)]
pub struct DynamicTriangle;

#[derive(Copy, Clone, Debug)]
pub enum AnyTriangle {
    Static = 0,
    Dynamic = 1,
}
