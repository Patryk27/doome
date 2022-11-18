use crate::*;

pub struct World<'a> {
    pub static_geo: &'a StaticGeometry,
    pub static_geo_index: &'a StaticGeometryIndex,
    pub dynamic_geo: &'a DynamicGeometry,
    pub mappings: &'a TriangleMappings,
    pub lights: &'a Lights,
    pub materials: &'a Materials,
    pub atlas_tex: &'a Image!(2D, type=f32, sampled),
    pub atlas_sampler: &'a Sampler,
}

impl<'a> World<'a> {
    pub fn geometry(&self, triangle_id: TriangleId<AnyTriangle>) -> Triangle {
        match triangle_id.unpack() {
            (AnyTriangle::Static, id) => {
                self.static_geo.get(TriangleId::new_static(id))
            }
            (AnyTriangle::Dynamic, id) => {
                self.dynamic_geo.get(TriangleId::new_dynamic(id))
            }
        }
    }

    pub fn atlas_sample(
        &self,
        triangle_id: TriangleId<AnyTriangle>,
        hit_uv: Vec2,
    ) -> Vec4 {
        let mapping = self.mappings.get(triangle_id);

        let tex_uv = mapping.uv0
            + (mapping.uv1 - mapping.uv0) * hit_uv.x
            + (mapping.uv2 - mapping.uv0) * hit_uv.y;

        self.atlas_tex
            .sample_by_lod(*self.atlas_sampler, tex_uv, 0.0)
    }
}
