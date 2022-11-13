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
        let inv_direction = 1.0 / self.direction;

        let hit_min_x = (bb_min.x - self.origin.x) * inv_direction.x;
        let hit_max_x = (bb_max.x - self.origin.x) * inv_direction.x;

        let hit_min_y = (bb_min.y - self.origin.y) * inv_direction.y;
        let hit_max_y = (bb_max.y - self.origin.y) * inv_direction.y;

        let hit_min_z = (bb_min.z - self.origin.z) * inv_direction.z;
        let hit_max_z = (bb_max.z - self.origin.z) * inv_direction.z;

        let x_entry = hit_min_x.min(hit_max_x);
        let y_entry = hit_min_y.min(hit_max_y);
        let z_entry = hit_min_z.min(hit_max_z);
        let x_exit = hit_min_x.max(hit_max_x);
        let y_exit = hit_min_y.max(hit_max_y);
        let z_exit = hit_min_z.max(hit_max_z);

        let latest_entry = x_entry.max(y_entry).max(z_entry);
        let earliest_exit = x_exit.min(y_exit).min(z_exit);

        latest_entry < earliest_exit && earliest_exit > 0.0
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

    pub fn trace(
        self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
    ) -> Hit {
        let mut hit = Hit::none();
        let mut nodes = PendingNodes::new();

        loop {
            let mut ptr = nodes.pop();

            if ptr == 0 {
                break;
            }

            let v1 = geometry_index.read(ptr);
            ptr += 1;

            let v2 = geometry_index.read(ptr);
            ptr += 1;

            let bb_min = v1.xyz();
            let left_len = v1.w as usize;

            let bb_max = v2.xyz();
            let triangles = v2.w as usize;

            if !self.hits_box(bb_min, bb_max) {
                continue;
            }

            if left_len == 0 {
                let mut triangle_idx = 0;

                while triangle_idx < triangles {
                    let triangle_id = geometry_index.read(ptr);
                    ptr += 1;

                    let triangle_id = triangle_id.x as u32;
                    let triangle = geometry.get(triangle_id);
                    let triangle_hit = triangle.hit(self);

                    if triangle_hit.is_closer_than(hit) {
                        hit = triangle_hit;
                    }

                    triangle_idx += 1;
                }
            } else {
                nodes.push(ptr);
                nodes.push(ptr + left_len);
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
            materials
                .get(hit.material_id)
                .shade_basic(geometry, lights, texture, hit)
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }
}
