mod loader;
mod model;

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use bevy::prelude::*;
use include_dir::Dir;

use self::loader::*;
pub(crate) use self::model::*;
use crate::components::ModelName;

#[derive(Resource)]
pub struct Assets {
    atlas: image::RgbaImage,
    models: HashMap<ModelName, Model>,
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
                let name = ModelName::new(file_stem.to_str().unwrap());

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

    pub(crate) fn model(&self, name: ModelName) -> &Model {
        self.models
            .get(&name)
            .unwrap_or_else(|| panic!("Unknown model: {}", name.name))
    }

    // pub fn insert_to_geometry(
    //     &self,
    //     model_handle: ModelHandle,
    //     xform: Mat4,
    //     alpha: f32,
    //     uv_transparency: bool,
    // ) {
    //     let model = &self.models[model_handle.0];

    //     for (triangle, triangle_mapping) in &model.triangles {
    //         let triangle = triangle
    //             .with_transform(xform)
    //             .with_alpha(alpha)
    //             .with_uv_transparency_of(uv_transparency);

    //         // TODO
    //         // geometry.push_ex(triangle, *triangle_mapping);
    //     }
    // }
}
