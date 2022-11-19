use std::path::{Path, PathBuf};

pub trait AssetsSource {
    fn read_file(&self, path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>>;
    fn list(&self) -> Vec<PathBuf>;
}

pub struct RuntimeSource(PathBuf);

impl RuntimeSource {
    pub fn new(p: impl AsRef<Path>) -> Self {
        Self(p.as_ref().to_owned())
    }
}

impl<'a> AssetsSource for &'a include_dir::Dir<'a> {
    fn read_file(&self, path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
        self.get_file(path.as_ref())
            .ok_or_else(|| anyhow::anyhow!("File not found"))
            .map(|file| file.contents().to_vec())
    }

    fn list(&self) -> Vec<PathBuf> {
        self.files()
            .map(|file| file.path().to_owned())
            .collect::<Vec<_>>()
    }
}

impl AssetsSource for RuntimeSource {
    fn read_file(&self, path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
        std::fs::read(self.0.join(path)).map_err(Into::into)
    }

    fn list(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for entry in std::fs::read_dir(&self.0).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                paths.push(path.strip_prefix(&self.0).unwrap().to_owned());
            }
        }
        paths
    }
}
