mod build;
mod load_material;
mod load_model;

use std::collections::HashMap;

use include_dir::Dir;

use super::{Assets, Model, ModelMaterial, ModelName, ModelTriangle};

pub struct AssetsLoader {
    dir: &'static Dir<'static>,
    models: HashMap<ModelName, Model>,

    /// Maps texture path to (texture data, models that use it)
    /// when building the pipeline we need to update the UVs on the models that use a given texture
    /// after packing it
    textures: HashMap<String, (image::RgbaImage, Vec<ModelName>)>,
}

impl AssetsLoader {
    pub fn new(dir: &'static Dir<'static>) -> Self {
        Self {
            dir,
            models: Default::default(),
            textures: Default::default(),
        }
    }
}
