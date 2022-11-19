use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub shaders: Vec<ShaderConfig>,
    pub dirs: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderConfig {
    pub path_to_shader: PathBuf,
    pub path_to_copy_to: PathBuf, // Without extension
}
