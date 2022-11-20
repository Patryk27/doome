use bevy::prelude::Entity;
use doome_raytracer as rt;
use glam::Mat4;

use super::GeometryManager;
use crate::assets::{Model, Texture};
use crate::components::{GeometryType, Material};

pub struct GeometryBuilder<'a> {
    geo: &'a mut GeometryManager,
}

impl<'a> GeometryBuilder<'a> {
    pub(super) fn new(geo: &'a mut GeometryManager) -> Self {
        Self { geo }
    }

    pub fn add_tri(
        &mut self,
        entity: Entity,
        geo_type: GeometryType,
        tex: Option<Texture>,
        tri: rt::Triangle,
        tri_uv: rt::TriangleMapping,
    ) {
        let tri_uv = tex.map(|tex| tex.map_uvs(tri_uv)).unwrap_or(tri_uv);

        match geo_type {
            GeometryType::Static => {
                self.geo.alloc_static(entity, tri, tri_uv);
            }
            GeometryType::Dynamic => {
                self.geo.alloc_dynamic(entity, tri, tri_uv);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_model(
        &mut self,
        entity: Entity,
        geo_type: GeometryType,
        model: &Model,
        xform: Mat4,
        mat: Material,
        mat_id: rt::MaterialId,
        tex: Option<Texture>,
    ) {
        for model_tri in &model.triangles {
            let tri = model_tri.materialize_triangle(xform, mat, mat_id);
            let tri_uv = model_tri.materialize_uvs();

            self.add_tri(entity, geo_type, tex, tri, tri_uv);
        }
    }
}
