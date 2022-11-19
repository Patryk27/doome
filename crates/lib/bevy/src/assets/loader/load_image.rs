use std::path::Path;

use anyhow::Context;

use super::source::AssetsSource;
use super::AssetsLoader;

impl<S> AssetsLoader<S>
where
    S: AssetsSource,
{
    pub fn load_image(
        &mut self,
        name: &str,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let image_data = self.source.read_file(path)?;

        let image = image::load_from_memory(&image_data)
            .context("Couldn't load image")?
            .to_rgba8();

        self.images.push(name, image);

        Ok(())
    }
}
