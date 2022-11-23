use doome_raytracer as rt;
use glam::{vec2, Mat4, Vec2, Vec3};
use rt::{ATLAS_HEIGHT, ATLAS_WIDTH};

use super::AssetHandle;
use crate::components::{Color, Material};

#[derive(Clone, Debug)]
pub struct Model {
    pub triangles: Vec<ModelTriangle>,
    pub material: ModelMaterial,
}

#[derive(Clone, Debug)]
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
        let (u_div, v_div) = mat.uv_divisor.unwrap_or((1, 1));

        rt::Triangle::new(
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
            mat_id,
        )
        .with_alpha(mat.alpha.unwrap_or(1.0))
        .with_transform(xform)
        .with_casts_shadows(mat.casts_shadows.unwrap_or(true))
        .with_uv_transparency(mat.uv_transparency.unwrap_or_default())
        .with_uv_divisor(u_div, v_div)
    }

    pub fn materialize_uvs(&self) -> rt::TriangleUv {
        rt::TriangleUv::new(self.uvs[0], self.uvs[1], self.uvs[2])
    }
}

#[derive(Clone, Debug, Default)]
pub struct ModelMaterial {
    pub color: Color,
    pub texture: Option<AssetHandle<Texture>>,
}

impl ModelMaterial {
    pub fn materialize(&self) -> Material {
        Material {
            color: Some(self.color),
            texture: self.texture,
            texture_enabled: Some(self.texture.is_some()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub atlas_offset_x: u32,
    pub atlas_offset_y: u32,
}

impl Texture {
    pub fn map_uvs(&self, uvs: rt::TriangleUv) -> rt::TriangleUv {
        rt::TriangleUv {
            uv0: self.map_uv(uvs.uv0),
            uv1: self.map_uv(uvs.uv1),
            uv2: self.map_uv(uvs.uv2),
        }
    }

    fn map_uv(&self, mut old_uv: Vec2) -> Vec2 {
        old_uv.y = 1.0 - old_uv.y;

        let pixel_coords = old_uv * vec2(self.width as _, self.height as _);
        let atlas_size = vec2(ATLAS_WIDTH as _, ATLAS_HEIGHT as _);
        let atlas_offset =
            vec2(self.atlas_offset_x as _, self.atlas_offset_y as _);

        (atlas_offset + pixel_coords) / atlas_size
    }
}
