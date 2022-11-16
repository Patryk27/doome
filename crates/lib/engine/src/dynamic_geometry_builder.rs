use raytracer as rt;

#[derive(Default)]
pub struct DynamicGeometryBuilder {
    geometry: rt::DynamicGeometry,
    geometry_mapping: rt::DynamicGeometryMapping,
}

impl DynamicGeometryBuilder {
    pub fn map_coords(&self, x: i32, z: i32) -> (f32, f32) {
        let x = (x as f32) * 2.0;
        let z = (z as f32) * 2.0;

        (x, z)
    }

    pub fn push(
        &mut self,
        triangle: rt::Triangle,
    ) -> rt::TriangleId<rt::DynamicTriangle> {
        self.geometry.push(triangle)
    }

    pub fn push_ex(
        &mut self,
        triangle: rt::Triangle,
        triangle_mapping: rt::TriangleMapping,
    ) -> rt::TriangleId<rt::DynamicTriangle> {
        let triangle_id = self.geometry.push(triangle);
        self.geometry_mapping.set(triangle_id, triangle_mapping);

        triangle_id
    }

    pub fn build(
        self,
    ) -> (Box<rt::DynamicGeometry>, Box<rt::DynamicGeometryMapping>) {
        (Box::new(self.geometry), Box::new(self.geometry_mapping))
    }
}
