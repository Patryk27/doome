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

    pub fn none() -> Self {
        Self::new(Default::default(), Default::default())
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn hits_box_at(self, bb_min: Vec3, bb_max: Vec3) -> f32 {
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

        if latest_entry <= earliest_exit && earliest_exit > 0.0 {
            latest_entry
        } else {
            f32::MAX
        }
    }

    pub fn hits_anything_up_to(self, world: &World, distance: f32) -> bool {
        // Check dynamic geometry
        let mut tri_idx = 0;

        while tri_idx < world.dynamic_geo.len() {
            let tri_id = TriangleId::new_dynamic(tri_idx);
            let tri = world.dynamic_geo.get(tri_id);
            let hit = tri.hit(self);

            if hit.t < distance {
                let got_hit = if tri.has_uv_transparency() {
                    world.atlas_sample(tri_id.into_any(), hit).w > 0.5
                } else {
                    true
                };

                if got_hit {
                    return true;
                }
            }

            tri_idx += 1;
        }

        // Check static geometry
        let mut ptr = 0;

        loop {
            let v1 = world.static_geo_index.read(ptr);
            let v2 = world.static_geo_index.read(ptr + 1);
            let is_leaf = v1.xyz() == v2.xyz();

            if is_leaf {
                let tri_id = TriangleId::new_static(v1.w as _);
                let tri = world.static_geo.get(tri_id);
                let hit = tri.hit(self);

                if hit.t < distance {
                    return true;
                }

                ptr = v2.w as _;
            } else {
                let at = self.hits_box_at(v1.xyz(), v2.xyz());

                if at < distance {
                    ptr = v1.w as _;
                } else {
                    ptr = v2.w as _;
                }
            }

            if ptr == 0 {
                break false;
            }
        }
    }

    pub fn trace(self, world: &World) -> Hit {
        let mut hit = Hit::none();

        // Check dynamic geometry
        let mut tri_idx = 0;

        while tri_idx < world.dynamic_geo.len() {
            let tri_id = TriangleId::new_dynamic(tri_idx);
            let tri = world.dynamic_geo.get(tri_id);
            let curr_hit = tri.hit(self);

            if curr_hit.is_closer_than(hit) {
                let got_hit = if tri.has_uv_transparency() {
                    world.atlas_sample(tri_id.into_any(), curr_hit).w > 0.5
                } else {
                    true
                };

                if got_hit {
                    hit = curr_hit;
                    hit.tri_id = tri_id.into_any();
                }
            }

            tri_idx += 1;
        }

        // Check static geometry
        let mut ptr = 0;

        loop {
            let v1 = world.static_geo_index.read(ptr);
            let v2 = world.static_geo_index.read(ptr + 1);
            let is_leaf = v1.xyz() == v2.xyz();

            if is_leaf {
                let tri_id = TriangleId::new_static(v1.w as _);
                let tri = world.static_geo.get(tri_id);
                let curr_hit = tri.hit(self);

                if curr_hit.is_closer_than(hit) {
                    hit = curr_hit;
                    hit.tri_id = tri_id.into_any();
                }

                ptr = v2.w as _;
            } else {
                let at = self.hits_box_at(v1.xyz(), v2.xyz());

                if at < hit.t {
                    ptr = v1.w as _;
                } else {
                    ptr = v2.w as _;
                }
            }

            if ptr == 0 {
                break;
            }
        }

        hit
    }

    pub fn shade(self, world: &World) -> Vec3 {
        let hit = self.trace(world);

        if hit.is_some() {
            world.materials.get(hit.mat_id).shade(world, hit)
        } else {
            Default::default()
        }
    }

    /// Shaders can't have recursive functions, so if we're processing a
    /// reflective surface, our code will call this function instead of
    /// [`Self::shade()`] to break the recursion-chain.
    pub fn shade_basic(self, world: &World) -> Vec3 {
        let hit = self.trace(world);

        if hit.is_some() {
            world.materials.get(hit.mat_id).shade_basic(world, hit)
        } else {
            Default::default()
        }
    }
}
