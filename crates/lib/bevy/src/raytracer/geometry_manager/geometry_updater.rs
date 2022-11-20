use bevy::prelude::Entity;
use doome_raytracer as rt;
use glam::Mat4;

use super::GeometryManager;
use crate::assets::{Model, Texture};
use crate::components::Material;

pub struct GeometryUpdater<'a> {
    geo: &'a mut GeometryManager,
}

impl<'a> GeometryUpdater<'a> {
    pub(super) fn new(geo: &'a mut GeometryManager) -> Self {
        Self { geo }
    }

    pub fn update_model(
        &mut self,
        entity: Entity,
        model: &Model,
        xform: Mat4,
        mat: Material,
        mat_id: rt::MaterialId,
        tex: Option<Texture>,
    ) {
        let mut tris = model.triangles.iter().map(|model_tri| {
            let tri = model_tri.materialize_triangle(xform, mat, mat_id);
            let tri_uv = model_tri.materialize_uvs();
            let tri_uv = tex.map(|tex| tex.map_uvs(tri_uv)).unwrap_or(tri_uv);

            (tri, tri_uv)
        });

        self.geo.update_dynamic(entity, |tri, tri_uv| {
            let (new_tri, new_tri_uv) = tris.next().unwrap();

            *tri = new_tri;
            *tri_uv = new_tri_uv;
        });
    }
}
