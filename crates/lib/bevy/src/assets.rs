mod loader;
mod model;
mod storage;

use std::path::Path;

use anyhow::{Context, Result};
use bevy::prelude::*;
use image::RgbaImage;
use include_dir::Dir;

use self::loader::*;
pub use self::model::*;
pub use self::storage::AssetHandle;
use self::storage::{AssetStorage, AssetStorageBuilder};
use crate::audio::Sound;

#[derive(Resource)]
pub struct Assets {
    atlas: image::RgbaImage,
    models: AssetStorage<Model>,
    images: AssetStorage<RgbaImage>,
    sounds: AssetStorage<Sound>,
    textures: AssetStorage<Texture>,
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

    fn init_inner(mut loader: AssetsLoader) -> Result<Self> {
        for (name, path) in loader.find("models", "png")? {
            loader.load_texture(&name, &path).with_context(|| {
                format!("Couldn't load texture: {}", path.display())
            })?;
        }

        for (name, path) in loader.find("models", "obj")? {
            loader.load_model(&name, &path).with_context(|| {
                format!("Couldn't load model: {}", path.display())
            })?;
        }

        for (name, path) in loader.find("images", "png")? {
            loader.load_image(&name, &path).with_context(|| {
                format!("Couldn't load image: {}", path.display())
            })?;
        }

        for (name, path) in loader.find("audio", "wav")? {
            loader.load_sound(&name, &path).with_context(|| {
                format!("Couldn't load image: {}", path.display())
            })?;
        }

        for (name, path) in loader.find("audio", "ogg")? {
            loader.load_sound(&name, &path).with_context(|| {
                format!("Couldn't load image: {}", path.display())
            })?;
        }

        for (name, path) in loader.find("audio", "mp3")? {
            loader.load_sound(&name, &path).with_context(|| {
                format!("Couldn't load image: {}", path.display())
            })?;
        }

        Ok(loader.build())
    }

    pub(crate) fn atlas(&self) -> &image::RgbaImage {
        &self.atlas
    }

    pub(crate) fn model(&self, handle: AssetHandle<Model>) -> &Model {
        self.models.by_handle(handle)
    }

    pub(crate) fn texture(&self, handle: AssetHandle<Texture>) -> Texture {
        *self.textures.by_handle(handle)
    }

    pub fn image(&self, handle: AssetHandle<RgbaImage>) -> &RgbaImage {
        self.images.by_handle(handle)
    }

    pub(crate) fn sound(&self, handle: AssetHandle<Sound>) -> &Sound {
        self.sounds.by_handle(handle)
    }

    pub fn load_model(&self, name: &str) -> AssetHandle<Model> {
        self.models
            .by_name(name)
            .unwrap_or_else(|| panic!("Unknown model: {}", name))
    }

    pub fn load_texture(&self, name: &str) -> AssetHandle<Texture> {
        self.textures
            .by_name(name)
            .unwrap_or_else(|| panic!("Unknown texture: {}", name))
    }

    pub fn load_image(&self, name: &str) -> AssetHandle<RgbaImage> {
        self.images
            .by_name(name)
            .unwrap_or_else(|| panic!("Unknown image: {}", name))
    }

    pub fn load_sound(&self, name: &str) -> AssetHandle<Sound> {
        self.sounds
            .by_name(name)
            .unwrap_or_else(|| panic!("Unknown sound: {}", name))
    }
}
