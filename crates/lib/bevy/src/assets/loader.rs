mod build;
mod load_material;
mod load_model;
mod source;

use std::collections::HashMap;
use std::path::PathBuf;

pub use self::source::{AssetsSource, RuntimeSource};
use super::{Assets, Model, ModelMaterial, ModelName, ModelTriangle};

pub struct AssetsLoader<S> {
    source: S,
    models: HashMap<ModelName, Model>,

    /// Maps texture path to (texture data, models that use it)
    /// when building the pipeline we need to update the UVs on the models that use a given texture
    /// after packing it
    textures: HashMap<String, (image::RgbaImage, Vec<ModelName>)>,
}

impl<S> AssetsLoader<S> {
    pub fn new(source: S) -> Self {
        Self {
            source,
            models: Default::default(),
            textures: Default::default(),
        }
    }
}

impl<S> AssetsLoader<S>
where
    S: AssetsSource,
{
    pub fn list(&self) -> Vec<PathBuf> {
        self.source.list()
    }
}
