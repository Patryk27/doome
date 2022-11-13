use crate::*;

#[derive(Copy, Clone, Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn hits_anything_up_to(
        self,
        geometry: &Geometry,
        distance: f32,
    ) -> bool {
        let mut triangle_idx = 0;

        while triangle_idx < geometry.len() {
            let hit = geometry.get(triangle_idx).hit(self);

            if hit.t < distance {
                return true;
            }

            triangle_idx += 1;
        }

        false
    }

    pub fn trace(self, geometry: &Geometry) -> Hit {
        let mut hit = Hit::none();
        let mut triangle_idx = 0;

        while triangle_idx < geometry.len() {
            let triangle = geometry.get(triangle_idx);
            let triangle_hit = triangle.hit(self);

            if triangle_hit.is_closer_than(hit) {
                hit = triangle_hit;
            }

            triangle_idx += 1;
        }

        hit
    }

    pub fn shade(
        self,
        geometry: &Geometry,
        lights: &Lights,
        materials: &Materials,
        texture: &Texture,
    ) -> Vec3 {
        let hit = self.trace(geometry);

        if hit.is_some() {
            materials
                .get(hit.material_id)
                .shade(geometry, lights, materials, texture, hit)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    /// Shaders can't have recursive functions, so if we're processing a
    /// reflective surface, our code will call _this_ function instead of
    /// [`Self::shade()`] to break the recursion-chain.
    ///
    /// There exist other techniques, too¹, but in our case they are (probably)
    /// not worth it.
    ///
    /// ¹ e.g. https://www.cs.uaf.edu/2012/spring/cs481/section/0/lecture/02_07_recursion_reflection.html
    pub fn shade_basic(
        self,
        geometry: &Geometry,
        lights: &Lights,
        materials: &Materials,
        texture: &Texture,
    ) -> Vec3 {
        let hit = self.trace(geometry);

        if hit.is_some() {
            materials
                .get(hit.material_id)
                .shade_basic(geometry, lights, texture, hit)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }
}
