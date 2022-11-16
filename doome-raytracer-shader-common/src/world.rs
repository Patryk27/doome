use crate::*;

pub struct World<'a> {
    pub geometry: &'a Geometry,
    pub geometry_mapping: &'a GeometryMapping,
    pub geometry_index: &'a GeometryIndex,
    pub lights: &'a Lights,
    pub materials: &'a Materials,
    pub atlas_tex: &'a Image!(2D, type=f32, sampled),
    pub atlas_sampler: &'a Sampler,
}

impl<'a> World<'a> {
    pub fn atlas_sample(&self, triangle_id: TriangleId, hit_uv: Vec2) -> Vec4 {
        let mapping = self.geometry_mapping.get(triangle_id);

        let tex_uv = mapping.uv0.xy()
            + (mapping.uv1.xy() - mapping.uv0.xy()) * hit_uv.x
            + (mapping.uv2.xy() - mapping.uv0.xy()) * hit_uv.y;

        self.atlas_tex
            .sample_by_lod(*self.atlas_sampler, tex_uv, 0.0)
    }
}
