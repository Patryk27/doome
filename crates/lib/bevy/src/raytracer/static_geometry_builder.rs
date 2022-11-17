use std::f32::consts::PI;

use doome_raytracer as rt;
use glam::vec3;

pub struct StaticGeometryBuilder<'a> {
    geometry: rt::StaticGeometry,
    mappings: &'a mut rt::TriangleMappings,
}

impl<'a> StaticGeometryBuilder<'a> {
    pub fn new(mappings: &'a mut rt::TriangleMappings) -> Self {
        Self {
            geometry: Default::default(),
            mappings,
        }
    }

    // TODO get rid of
    pub fn map_coords(&self, x: i32, z: i32) -> (f32, f32) {
        let x = (x as f32) * 2.0;
        let z = (z as f32) * 2.0;

        (x, z)
    }

    pub fn push(
        &mut self,
        triangle: rt::Triangle,
    ) -> rt::TriangleId<rt::StaticTriangle> {
        self.geometry.push(triangle)
    }

    pub fn push_ex(
        &mut self,
        triangle: rt::Triangle,
        triangle_mapping: rt::TriangleMapping,
    ) -> rt::TriangleId<rt::StaticTriangle> {
        let triangle_id = self.geometry.push(triangle);

        self.mappings.set(triangle_id.into_any(), triangle_mapping);

        triangle_id
    }

    pub fn push_floor(
        &mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        mat: rt::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = self.map_coords(x1, z1);
        let (x2, z2) = self.map_coords(x2, z2);

        self.push(rt::Triangle::new(
            vec3(x2, 0.0, z1),
            vec3(x1, 0.0, z1),
            vec3(x1, 0.0, z2),
            mat,
        ));

        self.push(rt::Triangle::new(
            vec3(x2, 0.0, z1),
            vec3(x1, 0.0, z2),
            vec3(x2, 0.0, z2),
            mat,
        ));
    }

    pub fn push_ceiling(
        &mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        mat: rt::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = self.map_coords(x1, z1);
        let (x2, z2) = self.map_coords(x2, z2);

        self.push(rt::Triangle::new(
            vec3(x2, 4.0, z1),
            vec3(x1, 4.0, z2),
            vec3(x1, 4.0, z1),
            mat,
        ));

        self.push(rt::Triangle::new(
            vec3(x2, 4.0, z1),
            vec3(x2, 4.0, z2),
            vec3(x1, 4.0, z2),
            mat,
        ));
    }

    pub fn push_wall(
        &mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        rot: u8,
        mat: rt::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = self.map_coords(x1, z1);
        let (x2, z2) = self.map_coords(x2, z2);
        let rot = (rot as f32) * (PI / 2.0);

        let vertex = |dx, y, dz| {
            let x = if dx < 0.0 { x1 } else { x2 };
            let z = if dz < 0.0 { z1 } else { z2 };

            vec3(x, y, z)
        };

        self.push(rt::Triangle::new(
            vertex(1.0 * rot.cos(), 0.0, -1.0 * rot.sin()),
            vertex(-1.0 * rot.cos(), 0.0, 1.0 * rot.sin()),
            vertex(-1.0 * rot.cos(), 4.0, 1.0 * rot.sin()),
            mat,
        ));

        self.push(rt::Triangle::new(
            vertex(1.0 * rot.cos(), 0.0, -1.0 * rot.sin()),
            vertex(-1.0 * rot.cos(), 4.0, 1.0 * rot.sin()),
            vertex(1.0 * rot.cos(), 4.0, -1.0 * rot.sin()),
            mat,
        ));
    }

    pub fn build(self) -> Box<rt::StaticGeometry> {
        Box::new(self.geometry)
    }
}
