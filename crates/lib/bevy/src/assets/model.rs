use doome_raytracer as rt;
use glam::{Mat4, Vec2, Vec3};

use crate::components::{Color, Material};

pub struct Model {
    pub triangles: Vec<ModelTriangle>,
    pub material: ModelMaterial,
}

pub struct ModelTriangle {
    pub vertices: [Vec3; 3],
    pub uvs: [Vec2; 3],
}

impl ModelTriangle {
    pub fn materialize_triangle(
        &self,
        xform: Mat4,
        mat: Material,
        mat_id: rt::MaterialId,
    ) -> rt::Triangle {
        rt::Triangle::new(
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
            mat_id,
        )
        .with_alpha(mat.alpha.unwrap_or(1.0))
        .with_transform(xform)
    }

    pub fn materialize_uvs(&self) -> rt::TriangleMapping {
        rt::TriangleMapping::new(self.uvs[0], self.uvs[1], self.uvs[2])
    }
}

#[derive(Default)]
pub struct ModelMaterial {
    pub color: Color,
    pub is_textured: bool,
}

impl ModelMaterial {
    pub fn materialize(&self) -> Material {
        Material {
            color: Some(self.color),
            is_textured: Some(self.is_textured),
            ..Default::default()
        }
    }
}
