use std::path::Path;

use anyhow::{Context, Result};

use super::AssetsLoader;

impl AssetsLoader {
    pub fn load_image(&mut self, name: &str, path: &Path) -> Result<()> {
        log::info!("Loading image: {}", path.display());

        let image = self.source.read_file(path)?;

        let image = image::load_from_memory(&image)
            .with_context(|| format!("Image is invalid: {}", path.display()))?
            .to_rgba8();

        self.images.push(name, image);

        Ok(())
    }
}
