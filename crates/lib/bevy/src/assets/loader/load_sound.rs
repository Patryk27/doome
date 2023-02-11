use std::path::Path;

use anyhow::Result;

use super::DoomeAssetsLoader;
use crate::audio::DoomeSound;

impl DoomeAssetsLoader {
    pub fn load_sound(&mut self, name: &str, path: &Path) -> Result<()> {
        log::info!("Loading sound: {}", path.display());

        let sound_data = self.source.read_file(path)?;

        self.sounds.push(name, DoomeSound::new(sound_data));

        Ok(())
    }
}
