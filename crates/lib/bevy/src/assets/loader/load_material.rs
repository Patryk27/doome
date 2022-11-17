use std::path::Path;

use tobj::{LoadError, MTLLoadResult};

use super::AssetsLoader;

impl AssetsLoader {
    pub fn load_material(&self, path: impl AsRef<Path>) -> MTLLoadResult {
        let path = path.as_ref();

        log::info!("Loading material: {}", path.display());

        let material_file =
            self.dir.get_file(path).ok_or(LoadError::OpenFileFailed)?;

        tobj::load_mtl_buf(&mut material_file.contents())
    }
}
