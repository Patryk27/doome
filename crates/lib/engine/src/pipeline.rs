use doome_raytracer as rt;
use glam::Mat4;
use include_dir::Dir;

pub use self::builder::PipelineBuilder;
use crate::static_geometry_builder::StaticGeometryBuilder;

mod builder;

pub struct Pipeline {
    models: Vec<Model>,
    atlas: image::RgbaImage,
}

impl Pipeline {
    pub fn builder(dir: Dir<'static>) -> PipelineBuilder {
        PipelineBuilder::new(dir)
    }

    pub fn atlas(&self) -> &image::RgbaImage {
        &self.atlas
    }

    pub fn insert_to_geometry(
        &self,
        model_handle: ModelHandle,
        geometry: &mut StaticGeometryBuilder,
        xform: Mat4,
        alpha: f32,
        uv_transparency: bool,
    ) {
        let model = &self.models[model_handle.0];

        for (triangle, triangle_mapping) in &model.triangles {
            let triangle = triangle
                .with_transform(xform)
                .with_alpha(alpha)
                .with_uv_transparency_of(uv_transparency);

            geometry.push_ex(triangle, *triangle_mapping);
        }
    }
}

#[derive(Clone)]
pub struct Model {
    triangles: Vec<(rt::Triangle, rt::TriangleMapping)>,
}

impl Model {
    pub fn new(triangles: Vec<(rt::Triangle, rt::TriangleMapping)>) -> Self {
        Self { triangles }
    }
}

#[derive(Clone, Copy)]
pub struct ModelHandle(usize);
