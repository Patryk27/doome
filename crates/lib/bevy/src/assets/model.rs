use doome_raytracer as rt;
use glam::{vec3, Mat4, Vec2, Vec3};

use crate::components::{Color, Reflective};

pub struct Model {
    pub triangles: Vec<ModelTriangle>,
    pub material: ModelMaterial,
}

pub struct ModelTriangle {
    pub vertices: [Vec3; 3],
    pub uvs: [Vec2; 3],
}

impl ModelTriangle {
    pub fn materialize(
        &self,
        mat: rt::MaterialId,
        alpha: f32,
        xform: Mat4,
    ) -> rt::Triangle {
        rt::Triangle::new(
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
            mat,
        )
        .with_alpha(alpha)
        .with_transform(xform)
    }
}

pub struct ModelMaterial {
    pub color: Vec3,
    pub is_textured: bool,
}

impl ModelMaterial {
    pub fn materialize(
        &self,
        color: Option<&Color>,
        reflect: Option<&Reflective>,
    ) -> rt::Material {
        let mut mat = rt::Material::default();

        if let Some(color) = color {
            mat = mat.with_color(color.into_vec3());
        } else {
            mat = mat.with_color(self.color).with_texture(self.is_textured);
        }

        if let Some(reflect) = reflect {
            mat = mat.with_reflectivity(
                reflect.reflectivity,
                reflect.reflection_color.into_vec3(),
            );
        }

        mat
    }
}

impl Default for ModelMaterial {
    fn default() -> Self {
        Self {
            color: vec3(1.0, 1.0, 1.0),
            is_textured: false,
        }
    }
}
