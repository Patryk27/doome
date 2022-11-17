use doome_raytracer as rt;

pub struct DynamicGeometryBuilder<'a> {
    geometry: rt::DynamicGeometry,
    mappings: &'a mut rt::TriangleMappings,
}

impl<'a> DynamicGeometryBuilder<'a> {
    pub fn new(mappings: &'a mut rt::TriangleMappings) -> Self {
        Self {
            geometry: Default::default(),
            mappings,
        }
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

        self.mappings.set(triangle_id.into_any(), triangle_mapping);

        triangle_id
    }

    pub fn build(self) -> Box<rt::DynamicGeometry> {
        Box::new(self.geometry)
    }
}
