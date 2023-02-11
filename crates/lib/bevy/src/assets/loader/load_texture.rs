use std::path::Path;

use anyhow::Result;

use super::DoomeAssetsLoader;

impl DoomeAssetsLoader {
    pub fn load_texture(&mut self, name: &str, path: &Path) -> Result<()> {
        log::info!("Loading texture: {}", path.display());

        let tex = self.source.read_file(path)?;
        let tex = image::load_from_memory(&tex)?.to_rgba8();

        self.textures.push(name, tex);

        Ok(())
    }
}
