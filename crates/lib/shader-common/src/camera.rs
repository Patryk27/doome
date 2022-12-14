use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Camera {
    pub origin: Vec4,
    pub look_at: Vec4,
    pub up: Vec4,
    pub viewport_size: Vec4,

    // Coordinates for the orthonormal basis; since they are somewhat heavy to
    // compute, we keep them here as a cache
    onb_u: Vec4,
    onb_v: Vec4,
    onb_w: Vec4,
}

impl Camera {
    pub fn origin(&self) -> Vec3 {
        self.origin.xyz()
    }

    pub fn ray(&self, pos: Vec2) -> Ray {
        let origin = self.origin.xyz();

        let direction = {
            let viewport_ratio = self.viewport_size.y / self.viewport_size.x;
            let viewport_fov = self.viewport_size.z;

            let pos = pos / self.viewport_size.xy(); // map to 0..1
            let pos = 2.0 * pos - 1.0; // map to -1..1
            let pos = vec2(pos.x / viewport_ratio, pos.y); // adjust for aspect ratio
            let pos = pos * (viewport_fov / 2.0).tan(); // adjust for the field of view

            OrthonormalBasis::trace(
                self.onb_u,
                self.onb_v,
                self.onb_w,
                vec4(pos.x, pos.y, -self.origin.w, 0.0),
            )
            .xyz()
        };

        Ray::new(origin, direction)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        distance: f32,
        viewport_size: Vec2,
        viewport_fov: f32,
    ) -> Self {
        let (onb_u, onb_v, onb_w) =
            OrthonormalBasis::build(origin, look_at, up);

        Self {
            origin: origin.extend(distance),
            look_at: look_at.extend(0.0),
            up: up.extend(0.0),
            viewport_size: viewport_size.extend(viewport_fov).extend(0.0),
            onb_u,
            onb_v,
            onb_w,
        }
    }

    pub fn update(&mut self, f: impl FnOnce(&mut Vec3, &mut Vec3, &mut Vec3)) {
        let mut origin = self.origin.xyz();
        let mut look_at = self.look_at.xyz();
        let mut up = self.up.xyz();

        f(&mut origin, &mut look_at, &mut up);

        let (onb_u, onb_v, onb_w) =
            OrthonormalBasis::build(origin, look_at, up);

        self.origin = origin.extend(self.origin.w);
        self.look_at = look_at.extend(0.0);
        self.up = up.extend(0.0);
        self.onb_u = onb_u;
        self.onb_v = onb_v;
        self.onb_w = onb_w;
    }
}

// Thanks to https://4programmers.net/Z_pogranicza/Raytracing
struct OrthonormalBasis;

impl OrthonormalBasis {
    fn build(eye: Vec3, look_at: Vec3, up: Vec3) -> (Vec4, Vec4, Vec4) {
        let w = (eye - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        (u.extend(0.0), v.extend(0.0), w.extend(0.0))
    }

    fn trace(u: Vec4, v: Vec4, w: Vec4, vec: Vec4) -> Vec4 {
        u * vec.x + v * vec.y + w * vec.z
    }
}
