use bevy::prelude::Entity;
use doome_raytracer as rt;
use glam::Mat4;

use super::GeometryManager;
use crate::assets::Model;
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
    ) {
        let mut tris = model.triangles.iter().map(|model_tri| {
            model_tri.materialize_triangle(xform, mat, mat_id)
        });

        self.geo.update_dynamic(entity, |tri| {
            *tri = tris.next().unwrap();
        });
    }
}
