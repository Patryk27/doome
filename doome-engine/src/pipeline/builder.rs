use std::collections::HashMap;
use std::path::Path;

use include_dir::Dir;

use super::{Model, ModelHandle};

mod build;
mod load_model;

pub struct PipelineBuilder {
    dir: Dir<'static>,
    models: Vec<Model>,

    /// Maps texture path to (texture data, models that use it)
    /// when building the pipeline we need to update the UVs on the models that use a given texture
    /// after packing it
    textures: HashMap<String, (image::RgbaImage, Vec<ModelHandle>)>,
}

impl PipelineBuilder {
    pub fn new(dir: Dir<'static>) -> Self {
        Self {
            dir,
            models: Vec::new(),
            textures: HashMap::new(),
        }
    }

    fn load_material(&self, path: impl AsRef<Path>) -> tobj::MTLLoadResult {
        let path = path.as_ref();

        log::info!("Loading material from {}", path.display());

        let material_file =
            self.dir.get_file(path).ok_or(tobj::LoadError::ReadError)?;

        let ret = tobj::load_mtl_buf(&mut material_file.contents())?;

        Ok(ret)
    }
}
