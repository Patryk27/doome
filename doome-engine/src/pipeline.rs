use std::path::Path;

use anyhow::anyhow;
use include_dir::{include_dir, Dir};
use tobj::LoadOptions;

const ASSETS: Dir = include_dir!("assets");

pub struct Pipeline {}

impl Pipeline {
    pub fn load_model(path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let model_file = ASSETS
            .get_file(path)
            .ok_or_else(|| anyhow!("Missing file {}", path.display()))?;

        let (model, materials) = tobj::load_obj_buf(
            &mut model_file.contents(),
            &LoadOptions::default(),
            |mat| Self::load_material(mat),
        )?;

        Ok(())
    }

    fn load_material(path: impl AsRef<Path>) -> tobj::MTLLoadResult {
        let path = path.as_ref();

        log::info!("Loading material from {}", path.display());

        let material_file =
            ASSETS.get_file(path).ok_or(tobj::LoadError::ReadError)?;

        let ret = tobj::load_mtl_buf(&mut material_file.contents())?;

        log::info!("Loaded material {ret:?}");

        Ok(ret)
    }
}
