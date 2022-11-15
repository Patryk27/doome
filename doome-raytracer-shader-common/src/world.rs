use crate::*;

pub struct World<'a> {
    pub geometry: &'a Geometry,
    pub geometry_index: &'a GeometryIndex,
    pub lights: &'a Lights,
    pub materials: &'a Materials,
    pub atlas_tex: &'a Image!(2D, type=f32, sampled),
    pub atlas_sampler: &'a Sampler,
}

impl<'a> World<'a> {
    pub fn atlas_sample(&self, uv: Vec2) -> Vec4 {
        self.atlas_tex.sample_by_lod(*self.atlas_sampler, uv, 0.0)
    }
}
