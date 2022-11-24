use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub shaders: Vec<ManifestShader>,
    pub extra_watch_dirs: Vec<PathBuf>,
}

impl Manifest {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let this = fs::read_to_string(path)?;
        let this = toml::from_str(&this)?;

        Ok(this)
    }

    pub fn build(&self) -> Result<()> {
        for shader in &self.shaders {
            shader.build()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct ManifestShader {
    pub source_dir: PathBuf,
    pub target_dir: PathBuf,
}

impl ManifestShader {
    pub fn build(&self) -> Result<()> {
        let compile_result =
            SpirvBuilder::new(&self.source_dir, "spirv-unknown-spv1.5")
                .print_metadata(MetadataPrintout::None)
                .capability(Capability::Int8)
                .release(true)
                .build()?;

        if let Some(parent) = self.target_dir.parent() {
            fs::create_dir_all(parent)?;
        }

        let spirv_module_path = self.target_dir.with_extension("spv");
        let glsl_module_path = self.target_dir.with_extension("glsl");

        fs_extra::file::copy(
            compile_result.module.unwrap_single(),
            &spirv_module_path,
            &fs_extra::file::CopyOptions {
                overwrite: true,
                skip_exist: false,
                ..fs_extra::file::CopyOptions::default()
            },
        )?;

        // TODO that's a hack so that we can inspect the raytracer's glsl output
        {
            let module = fs::read(spirv_module_path)?;

            let module =
                naga::front::spv::parse_u8_slice(&module, &Default::default())?;

            let info = naga::valid::Validator::new(
                Default::default(),
                Default::default(),
            )
            .validate(&module)?;

            let policies = naga::proc::BoundsCheckPolicies {
                index: naga::proc::BoundsCheckPolicy::Unchecked,
                buffer: naga::proc::BoundsCheckPolicy::Unchecked,
                image: naga::proc::BoundsCheckPolicy::Unchecked,
                binding_array: naga::proc::BoundsCheckPolicy::Unchecked,
            };

            let opts = Default::default();

            let pipeline_options = naga::back::glsl::PipelineOptions {
                shader_stage: naga::ShaderStage::Fragment,
                entry_point: "fs_main".into(),
                multiview: None,
            };

            let mut output = String::new();

            let writer = naga::back::glsl::Writer::new(
                &mut output,
                &module,
                &info,
                &opts,
                &pipeline_options,
                policies,
            );

            if let Ok(mut writer) = writer {
                writer.write()?;

                fs::write(glsl_module_path, output)?;
            }
        }

        Ok(())
    }
}
