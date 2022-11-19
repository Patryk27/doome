mod loader;
mod model;

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use bevy::prelude::*;
use include_dir::Dir;

use self::loader::*;
pub(crate) use self::model::*;
use crate::components::ModelHandle;

#[derive(Resource)]
pub struct Assets {
    atlas: image::RgbaImage,
    models: Vec<Model>,
    name_to_index: HashMap<String, usize>,
}

impl Assets {
    pub fn init_static(dir: &'static Dir<'static>) -> Result<Self> {
        let loader = AssetsLoader::new(dir);
        Self::init_inner(loader)
    }

    pub fn init(path: impl AsRef<Path>) -> Result<Self> {
        let runtime_source = RuntimeSource::new(path);
        let loader = AssetsLoader::new(runtime_source);
        Self::init_inner(loader)
    }

    fn init_inner<S: AssetsSource>(
        mut loader: AssetsLoader<S>,
    ) -> Result<Self> {
        for entry in loader.list() {
            let Some(entry_ext) = entry.extension() else { continue };

            if entry_ext.to_str() == Some("obj") {
                let file_stem = entry.file_stem().unwrap();
                let name = file_stem.to_str().unwrap();

                loader.load_model(name, &entry).with_context(|| {
                    format!("Couldn't load model: {}", entry.display())
                })?;
            }
        }

        Ok(loader.build())
    }

    pub(crate) fn atlas(&self) -> &image::RgbaImage {
        &self.atlas
    }

    pub(crate) fn model(&self, handle: ModelHandle) -> &Model {
        &self.models[handle.0]
    }

    /// "Loads" a model and returns a handle to it
    ///
    /// Panics if the given model doesn't exist
    pub fn load_model(&self, name: &str) -> ModelHandle {
        let index = self.name_to_index.get(name).expect("Failed to load model");
        ModelHandle(*index)
    }
}
