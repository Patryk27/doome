mod loader;
pub mod model;

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use bevy::prelude::*;
use image::RgbaImage;
use include_dir::Dir;

use self::loader::*;
pub(crate) use self::model::*;

#[derive(Resource)]
pub struct Assets {
    atlas: image::RgbaImage,
    models: AssetStorage<Model>,
    images: AssetStorage<RgbaImage>,
}

pub struct AssetStorage<T> {
    items: Vec<T>,
    name_to_index: HashMap<String, usize>,
}

pub struct AssetStorageBuilder<T> {
    items: Vec<T>,
    name_to_index: HashMap<String, usize>,
}

#[derive(Component)]
pub struct AssetHandle<T>(usize, std::marker::PhantomData<T>);

impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        Self(self.0, std::marker::PhantomData)
    }
}

impl<T> Copy for AssetHandle<T> {}

impl<T> AssetHandle<T> {
    pub(crate) fn new(index: usize) -> Self {
        Self(index, std::marker::PhantomData)
    }
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
            let Some(ext) = entry_ext.to_str() else { continue };
            let Some(stem) = entry.file_stem() else { continue };
            let Some(name) = stem.to_str() else { continue };

            match ext {
                "obj" => {
                    loader.load_model(name, &entry).with_context(|| {
                        format!("Couldn't load model: {}", entry.display())
                    })?
                }
                "png" => {
                    loader.load_image(name, &entry).with_context(|| {
                        format!("Couldn't load image: {}", entry.display())
                    })?
                }
                _ => (),
            }
        }

        Ok(loader.build())
    }

    pub(crate) fn atlas(&self) -> &image::RgbaImage {
        &self.atlas
    }

    pub(crate) fn model(&self, handle: AssetHandle<Model>) -> &Model {
        self.models.by_handle(handle)
    }

    pub fn image(&self, handle: AssetHandle<RgbaImage>) -> &RgbaImage {
        self.images.by_handle(handle)
    }

    pub fn load_model(&self, name: &str) -> AssetHandle<Model> {
        self.models.by_name(name).expect("Failed to load model")
    }

    pub fn load_image(&self, name: &str) -> AssetHandle<RgbaImage> {
        self.images.by_name(name).expect("Failed to load image")
    }
}

impl<T> AssetStorage<T> {
    fn by_name(&self, name: &str) -> Option<AssetHandle<T>> {
        let index = self.name_to_index.get(name)?;
        Some(AssetHandle::new(*index))
    }

    fn by_handle(&self, handle: AssetHandle<T>) -> &T {
        &self.items[handle.0]
    }
}

impl<T> AssetStorageBuilder<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            name_to_index: Default::default(),
        }
    }

    fn push(&mut self, name: &str, item: T) {
        let index = self.items.len();
        self.items.push(item);
        self.name_to_index.insert(name.to_string(), index);
    }

    fn modify(&mut self, name: &str) -> &mut T {
        let index = self.name_to_index[name];
        &mut self.items[index]
    }

    fn build(self) -> AssetStorage<T> {
        AssetStorage {
            items: self.items,
            name_to_index: self.name_to_index,
        }
    }
}
