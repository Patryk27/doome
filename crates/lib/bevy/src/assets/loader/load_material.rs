use std::path::Path;

use tobj::{LoadError, MTLLoadResult};

use super::source::AssetsSource;
use super::AssetsLoader;

impl<S> AssetsLoader<S>
where
    S: AssetsSource,
{
    pub fn load_material(&self, path: impl AsRef<Path>) -> MTLLoadResult {
        let path = path.as_ref();

        log::info!("Loading material: {}", path.display());

        let material_file = self.source.read_file(path).map_err(|err| {
            log::error!("Error loading material: {err:?}");
            LoadError::OpenFileFailed
        })?;

        tobj::load_mtl_buf(&mut material_file.as_slice())
    }
}
