use std::collections::HashMap;
use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct AssetHandle<T>(usize, PhantomData<T>);

impl<T> AssetHandle<T> {
    pub(super) fn new(index: usize) -> Self {
        Self(index, PhantomData)
    }

    pub(super) fn id(self) -> usize {
        self.0
    }

    pub(super) fn transmute<U>(self) -> AssetHandle<U> {
        AssetHandle(self.0, PhantomData)
    }
}

impl<T> Clone for AssetHandle<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T> Copy for AssetHandle<T> {}

impl<T> PartialEq for AssetHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub struct AssetStorage<T> {
    items: Vec<T>,
    index: HashMap<String, usize>,
}

impl<T> AssetStorage<T> {
    pub fn by_name(&self, name: &str) -> Option<AssetHandle<T>> {
        self.index.get(name).cloned().map(AssetHandle::new)
    }

    pub fn by_handle(&self, handle: AssetHandle<T>) -> &T {
        &self.items[handle.0]
    }
}

pub struct AssetStorageBuilder<T> {
    items: Vec<T>,
    index: HashMap<String, usize>,
}

impl<T> AssetStorageBuilder<T> {
    pub fn push(&mut self, name: &str, item: T) {
        let index = self.items.len();

        self.items.push(item);
        self.index.insert(name.to_string(), index);
    }

    pub fn try_by_name(&self, name: &str) -> Option<AssetHandle<T>> {
        self.index.get(name).cloned().map(AssetHandle::new)
    }

    pub fn by_handle(&self, handle: AssetHandle<T>) -> (&T, &str) {
        let item = &self.items[handle.0];

        let name = self
            .index
            .iter()
            .find(|(_, id)| **id == handle.0)
            .map(|(name, _)| name)
            .unwrap();

        (item, name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (AssetHandle<T>, &T)> + '_ {
        self.items
            .iter()
            .enumerate()
            .map(|(id, item)| (AssetHandle::new(id), item))
    }

    pub fn build(self) -> AssetStorage<T> {
        AssetStorage {
            items: self.items,
            index: self.index,
        }
    }
}

impl<T> Default for AssetStorageBuilder<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
            index: Default::default(),
        }
    }
}
