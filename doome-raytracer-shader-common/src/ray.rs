use crate::*;

#[derive(Copy, Clone, Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    inv_direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let direction = direction.normalize();

        Self {
            origin,
            direction,
            inv_direction: 1.0 / direction,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    // TODO optimizable
    pub fn hits_box(&self, bb_min: Vec3, bb_max: Vec3) -> bool {
        let hit_min = (bb_min - self.origin) * self.inv_direction;
        let hit_max = (bb_max - self.origin) * self.inv_direction;

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
        visitor: &mut GeometryIndexVisitor,
        distance: f32,
    ) -> bool {
        let hit = self.trace(geometry, geometry_index, visitor);

        // TODO optimizable
        hit.is_some() && hit.t < distance
    }

    pub fn trace(
        self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
        visitor: &mut GeometryIndexVisitor,
    ) -> Hit {
        let mut hit = Hit::none();

        visitor.reset();

        loop {
            let mut ptr = visitor.pop();

            if ptr == 0 {
                break;
            }

            let bb_min = geometry_index.read(ptr);
            let bb_max = geometry_index.read(ptr + 1);

            ptr += 2;

            let left_len = bb_min.w as usize;

            if left_len == 0 {
                let triangles = bb_max.w as usize;
                let mut triangle_idx = 0;

                while triangle_idx < triangles {
                    let triangle_id = geometry_index.read(ptr);

                    ptr += 1;
                    triangle_idx += 4;

                    // Check .x
                    let triangle_hit =
                        geometry.get(triangle_id.x as _).hit(self);

                    if triangle_hit.is_closer_than(hit) {
                        hit = triangle_hit;
                    }

                    // Check .y
                    let triangle_hit =
                        geometry.get(triangle_id.y as _).hit(self);

                    if triangle_hit.is_closer_than(hit) {
                        hit = triangle_hit;
                    }

                    // Check .z
                    let triangle_hit =
                        geometry.get(triangle_id.z as _).hit(self);

                    if triangle_hit.is_closer_than(hit) {
                        hit = triangle_hit;
                    }

                    // Check .w
                    let triangle_hit =
                        geometry.get(triangle_id.w as _).hit(self);

                    if triangle_hit.is_closer_than(hit) {
                        hit = triangle_hit;
                    }
                }
            } else {
                // TODO prioritize left/right depending on the distance

                let left_bb_min = geometry_index.read(ptr);
                let left_bb_max = geometry_index.read(ptr + 1);

                if self.hits_box(left_bb_min.xyz(), left_bb_max.xyz()) {
                    visitor.push(ptr);
                }

                let right_bb_min = geometry_index.read(ptr + left_len);
                let right_bb_max = geometry_index.read(ptr + left_len + 1);

                if self.hits_box(right_bb_min.xyz(), right_bb_max.xyz()) {
                    visitor.push(ptr + left_len);
                }
            }
        }

        hit
    }

    pub fn shade(
        self,
        geometry: &Geometry,
        geometry_index: &GeometryIndex,
        geometry_index_visitor: &mut GeometryIndexVisitor,
        lights: &Lights,
        materials: &Materials,
        texture: &Texture,
    ) -> Vec3 {
        let hit = self.trace(geometry, geometry_index, geometry_index_visitor);

        if hit.is_some() {
            materials.get(hit.material_id).shade(
                geometry,
                geometry_index,
                geometry_index_visitor,
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
        geometry_index_visitor: &mut GeometryIndexVisitor,
        lights: &Lights,
        materials: &Materials,
        texture: &Texture,
    ) -> Vec3 {
        let hit = self.trace(geometry, geometry_index, geometry_index_visitor);

        if hit.is_some() {
            materials.get(hit.material_id).shade_basic(
                geometry,
                geometry_index,
                geometry_index_visitor,
                lights,
                texture,
                hit,
            )
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }
}
