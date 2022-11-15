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

    pub fn hits_box(&self, bb_min: Vec3, bb_max: Vec3) -> bool {
        let hit_min = (bb_min - self.origin) / self.direction;
        let hit_max = (bb_max - self.origin) / self.direction;

        let x_entry = hit_min.x.min(hit_max.x);
        let y_entry = hit_min.y.min(hit_max.y);
        let z_entry = hit_min.z.min(hit_max.z);
        let x_exit = hit_min.x.max(hit_max.x);
        let y_exit = hit_min.y.max(hit_max.y);
        let z_exit = hit_min.z.max(hit_max.z);

        let latest_entry = x_entry.max(y_entry).max(z_entry);
        let earliest_exit = x_exit.min(y_exit).min(z_exit);

        latest_entry <= earliest_exit && earliest_exit > 0.0
    }

    pub fn hits_anything_up_to(
        self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
        distance: f32,
    ) -> bool {
        let mut ptr = 0;

        loop {
            let v1 = geometry_index.read(ptr);
            let v2 = geometry_index.read(ptr + 1);

            let is_leaf = v1.xyz() == vec3(0.0, 0.0, 0.0)
                && v2.xyz() == vec3(0.0, 0.0, 0.0);

            if is_leaf {
                let hit = geometry.get(v1.w as _).hit(self);

                if hit.t < distance {
                    return true;
                }

                ptr = v2.w as _;
            } else if self.hits_box(v1.xyz(), v2.xyz()) {
                ptr = v1.w as _;
            } else {
                ptr = v2.w as _;
            }

            if ptr == 0 {
                break false;
            }
        }
    }

    pub fn trace(
        self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
    ) -> Hit {
        let mut hit = Hit::none();
        let mut ptr = 0;

        loop {
            let v1 = geometry_index.read(ptr);
            let v2 = geometry_index.read(ptr + 1);

            let is_leaf = v1.xyz() == vec3(0.0, 0.0, 0.0)
                && v2.xyz() == vec3(0.0, 0.0, 0.0);

            if is_leaf {
                let curr_hit = geometry.get(v1.w as _).hit(self);

                if curr_hit.is_closer_than(hit) {
                    hit = curr_hit;
                }

                ptr = v2.w as _;
            } else if self.hits_box(v1.xyz(), v2.xyz()) {
                ptr = v1.w as _;
            } else {
                ptr = v2.w as _;
            }

            if ptr == 0 {
                break;
            }
        }

        hit
    }

    pub fn shade(
        self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
        lights: &Lights,
        materials: &Materials,
        texture: &Texture,
    ) -> Vec3 {
        let hit = self.trace(geometry, geometry_index);

        if hit.is_some() {
            materials.get(hit.material_id).shade(
                geometry,
                geometry_index,
                lights,
                materials,
                texture,
                hit,
            )
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
        geometry_index: &GeometryIndex,
        lights: &Lights,
        materials: &Materials,
        texture: &Texture,
    ) -> Vec3 {
        let hit = self.trace(geometry, geometry_index);

        if hit.is_some() {
            materials.get(hit.material_id).shade_basic(
                geometry,
                geometry_index,
                lights,
                texture,
                hit,
            )
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }
}
