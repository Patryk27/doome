use crate::*;

pub struct World<'a> {
    pub static_geo: &'a StaticGeometry,
    pub static_geo_mapping: &'a StaticGeometryMapping,
    pub static_geo_index: &'a StaticGeometryIndex,
    pub dynamic_geo: &'a DynamicGeometry,
    pub dynamic_geo_mapping: &'a DynamicGeometryMapping,
    pub lights: &'a Lights,
    pub materials: &'a Materials,
    pub atlas_tex: &'a Image!(2D, type=f32, sampled),
    pub atlas_sampler: &'a Sampler,
}

impl<'a> World<'a> {
    pub fn geometry(&self, triangle_id: TriangleId<AnyTriangle>) -> Triangle {
        match triangle_id.get() {
            (AnyTriangle::Static, id) => {
                self.static_geo.get(TriangleId::new_static(id))
            }
            (AnyTriangle::Dynamic, id) => {
                self.dynamic_geo.get(TriangleId::new_dynamic(id))
            }
        }
    }

    pub fn geometry_mapping(
        &self,
        triangle_id: TriangleId<AnyTriangle>,
    ) -> TriangleMapping {
        match triangle_id.get() {
            (AnyTriangle::Static, id) => {
                self.static_geo_mapping.get(TriangleId::new_static(id))
            }
            (AnyTriangle::Dynamic, id) => {
                self.dynamic_geo_mapping.get(TriangleId::new_dynamic(id))
            }
        }
    }

    pub fn atlas_sample(
        &self,
        triangle_id: TriangleId<AnyTriangle>,
        hit_uv: Vec2,
    ) -> Vec4 {
        let mapping = self.geometry_mapping(triangle_id);

        let tex_uv = mapping.uv0.xy()
            + (mapping.uv1.xy() - mapping.uv0.xy()) * hit_uv.x
            + (mapping.uv2.xy() - mapping.uv0.xy()) * hit_uv.y;

        self.atlas_tex
            .sample_by_lod(*self.atlas_sampler, tex_uv, 0.0)
    }
}
