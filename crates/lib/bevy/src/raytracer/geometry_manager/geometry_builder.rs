use std::f32::consts::PI;

use bevy::prelude::Entity;
use doome_raytracer as rt;
use glam::{vec3, Mat4};

use super::GeometryManager;
use crate::assets::Model;
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
        triangle: rt::Triangle,
    ) {
        self.add_tri_uv(entity, geo_type, triangle, Default::default());
    }

    pub fn add_tri_uv(
        &mut self,
        entity: Entity,
        geo_type: GeometryType,
        triangle: rt::Triangle,
        triangle_uv: rt::TriangleMapping,
    ) {
        match geo_type {
            GeometryType::Static => {
                self.geo.alloc_static(entity, triangle, triangle_uv);
            }
            GeometryType::Dynamic => {
                self.geo.alloc_dynamic(entity, triangle, triangle_uv);
            }
        }
    }

    pub fn add_floor(
        &mut self,
        entity: Entity,
        geo_type: GeometryType,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        mat: rt::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = (x1 as f32, z1 as f32);
        let (x2, z2) = (x2 as f32, z2 as f32);

        self.add_tri(
            entity,
            geo_type,
            rt::Triangle::new(
                vec3(x2, 0.0, z1),
                vec3(x1, 0.0, z1),
                vec3(x1, 0.0, z2),
                mat,
            ),
        );

        self.add_tri(
            entity,
            geo_type,
            rt::Triangle::new(
                vec3(x2, 0.0, z1),
                vec3(x1, 0.0, z2),
                vec3(x2, 0.0, z2),
                mat,
            ),
        );
    }

    pub fn ceiling(
        &mut self,
        entity: Entity,
        geo_type: GeometryType,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        mat: rt::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = (x1 as f32, z1 as f32);
        let (x2, z2) = (x2 as f32, z2 as f32);

        self.add_tri(
            entity,
            geo_type,
            rt::Triangle::new(
                vec3(x2, 4.0, z1),
                vec3(x1, 4.0, z2),
                vec3(x1, 4.0, z1),
                mat,
            ),
        );

        self.add_tri(
            entity,
            geo_type,
            rt::Triangle::new(
                vec3(x2, 4.0, z1),
                vec3(x2, 4.0, z2),
                vec3(x1, 4.0, z2),
                mat,
            ),
        );
    }

    pub fn add_wall(
        &mut self,
        entity: Entity,
        geo_type: GeometryType,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        rot: u8,
        mat: rt::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = (x1 as f32, z1 as f32);
        let (x2, z2) = (x2 as f32, z2 as f32);
        let rot = (rot as f32) * (PI / 2.0);

        let vertex = |dx, y, dz| {
            let x = if dx < 0.0 { x1 } else { x2 };
            let z = if dz < 0.0 { z1 } else { z2 };

            vec3(x, y, z)
        };

        self.add_tri(
            entity,
            geo_type,
            rt::Triangle::new(
                vertex(1.0 * rot.cos(), 0.0, -1.0 * rot.sin()),
                vertex(-1.0 * rot.cos(), 0.0, 1.0 * rot.sin()),
                vertex(-1.0 * rot.cos(), 4.0, 1.0 * rot.sin()),
                mat,
            ),
        );

        self.add_tri(
            entity,
            geo_type,
            rt::Triangle::new(
                vertex(1.0 * rot.cos(), 0.0, -1.0 * rot.sin()),
                vertex(-1.0 * rot.cos(), 4.0, 1.0 * rot.sin()),
                vertex(1.0 * rot.cos(), 4.0, -1.0 * rot.sin()),
                mat,
            ),
        );
    }

    pub fn add_model(
        &mut self,
        entity: Entity,
        geo_type: GeometryType,
        model: &Model,
        xform: Mat4,
        mat: Material,
        mat_id: rt::MaterialId,
    ) {
        for model_tri in &model.triangles {
            let tri = model_tri.materialize_triangle(xform, mat, mat_id);
            let tri_uv = model_tri.materialize_uvs();

            self.add_tri_uv(entity, geo_type, tri, tri_uv);
        }
    }
}
