use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

pub trait AssetsSource {
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
    fn read_file(&self, path: &Path) -> Result<Vec<u8>>;
}

impl AssetsSource for &'static include_dir::Dir<'static> {
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        self.get_dir(path)
            .ok_or_else(|| anyhow!("Directory not found: {}", path.display()))
            .map(|dir| dir.files().map(|file| file.path().to_owned()).collect())
    }

    fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        self.get_file(path)
            .ok_or_else(|| anyhow!("File not found: {}", path.display()))
            .map(|file| file.contents().to_vec())
    }
}

pub struct RuntimeSource(PathBuf);

impl RuntimeSource {
    pub fn new(p: impl AsRef<Path>) -> Self {
        Self(p.as_ref().to_owned())
    }
}

impl AssetsSource for RuntimeSource {
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let entries = fs::read_dir(self.0.join(path)).with_context(|| {
            format!("Couldn't read directory: {}", path.display())
        })?;

        let mut paths = Vec::new();

        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                paths.push(path.strip_prefix(&self.0).unwrap().to_owned());
            }
        }

        Ok(paths)
    }

    fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        fs::read(self.0.join(path))
            .with_context(|| format!("Couldn't read file: {}", path.display()))
    }
}
