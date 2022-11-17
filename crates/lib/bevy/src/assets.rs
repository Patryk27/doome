mod loader;

use std::collections::HashMap;

use anyhow::{Context, Result};
use bevy::prelude::*;
use doome_raytracer as rt;
use include_dir::Dir;

use self::loader::*;
use crate::components::ModelName;

#[derive(Resource)]
pub struct Assets {
    atlas: image::RgbaImage,
    models: HashMap<ModelName, Model>,
}

impl Assets {
    pub fn init(dir: &'static Dir<'static>) -> Result<Self> {
        let mut loader = AssetsLoader::new(dir);

        for entry in dir.entries() {
            let Some(entry) = entry.as_file() else { continue };
            let Some(entry_ext) = entry.path().extension() else { continue };

            if entry_ext.to_str() == Some("obj") {
                let name = ModelName::new(
                    entry.path().file_stem().unwrap().to_str().unwrap(),
                );

                loader
                    .load_model(
                        name,
                        entry.path(),
                        // TODO
                        rt::MaterialId::new(0),
                    )
                    .with_context(|| {
                        format!(
                            "Couldn't load model: {}",
                            entry.path().display()
                        )
                    })?;
            }
        }

        Ok(loader.build())
    }

    pub fn atlas(&self) -> &image::RgbaImage {
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

pub struct Model {
    pub triangles: Vec<(rt::Triangle, rt::TriangleMapping)>,
}

impl Model {
    fn new(triangles: Vec<(rt::Triangle, rt::TriangleMapping)>) -> Self {
        Self { triangles }
    }
}
