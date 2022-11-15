use doome_raytracer as rt;
use glam::Mat4;
use include_dir::Dir;

pub use self::builder::PipelineBuilder;
use crate::geometry_builder::GeometryBuilder;

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
        geometry: &mut GeometryBuilder,
        xform: Mat4,
    ) {
        let model = &self.models[model_handle.0];

        for (triangle, triangle_mapping) in &model.triangles {
            geometry.push_ex(triangle.apply(xform), *triangle_mapping);
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
