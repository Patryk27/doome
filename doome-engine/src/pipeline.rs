use doome_raytracer as rt;
use glam::Mat4;
use include_dir::Dir;

pub use self::builder::PipelineBuilder;

mod builder;

#[derive(Clone, Copy)]
pub struct ModelHandle(usize);

pub struct Pipeline {
    models: Vec<Vec<rt::Triangle>>,
    atlas: image::RgbaImage,
}

impl Pipeline {
    pub fn builder(dir: &'static Dir) -> PipelineBuilder {
        PipelineBuilder::new(dir)
    }

    pub fn atlas(&self) -> &image::RgbaImage {
        &self.atlas
    }

    pub fn insert_to_geometry(
        &self,
        model_handle: ModelHandle,
        geometry: &mut rt::Geometry,
        xform: Mat4,
    ) -> u32 {
        let model = &self.models[model_handle.0];
        let offset = geometry.len();

        for tri in model {
            geometry.push(tri.apply(xform));
        }

        offset
    }

    pub fn update_geometry(
        &self,
        offset: u32,
        model_handle: ModelHandle,
        geometry: &mut rt::Geometry,
        xform: Mat4,
    ) {
        let triangles: Vec<_> = self.models[model_handle.0]
            .iter()
            .map(|tri| tri.apply(xform))
            .collect();

        geometry.write(offset, &triangles);
    }
}
