use std::path::Path;

use anyhow::Result;

use super::AssetsLoader;
use crate::audio::Sound;

impl AssetsLoader {
    pub fn load_sound(&mut self, name: &str, path: &Path) -> Result<()> {
        log::info!("Loading sound: {}", path.display());

        let sound_data = self.source.read_file(path)?;

        self.sounds.push(name, Sound::new(sound_data));

        Ok(())
    }
}
