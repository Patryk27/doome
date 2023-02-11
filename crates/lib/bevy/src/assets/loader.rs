mod build;
mod load_image;
mod load_material;
mod load_model;
mod load_texture;
mod source;

use std::path::{Path, PathBuf};

use anyhow::Result;
use image::RgbaImage;

pub use self::source::{AssetsSource, RuntimeSource};
use super::{
    AssetStorageBuilder, DoomeAssetHandle, DoomeAssets, Model, ModelMaterial,
    ModelTriangle, Texture,
};

pub struct DoomeAssetsLoader {
    source: Box<dyn AssetsSource>,
    models: AssetStorageBuilder<Model>,
    images: AssetStorageBuilder<RgbaImage>,
    textures: AssetStorageBuilder<RgbaImage>,
}

impl DoomeAssetsLoader {
    pub fn new(source: impl AssetsSource + 'static) -> Self {
        Self {
            source: Box::new(source),
            models: Default::default(),
            images: Default::default(),
            textures: Default::default(),
        }
    }

    pub fn find(
        &self,
        match_dir: &str,
        match_ext: &str,
    ) -> Result<Vec<(String, PathBuf)>> {
        let mut entries = Vec::new();

        for entry in self.source.read_dir(Path::new(match_dir))? {
            let Some(ext) = entry.extension() else { continue };
            let Some(ext) = ext.to_str() else { continue };
            let Some(stem) = entry.file_stem() else { continue };
            let Some(stem) = stem.to_str() else { continue };

            if ext != match_ext {
                continue;
            }

            entries.push((stem.to_owned(), entry));
        }

        Ok(entries)
    }
}
