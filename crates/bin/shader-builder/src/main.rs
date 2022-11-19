mod manifest;

use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use manifest::{Manifest, ShaderConfig};
use notify::Watcher;
use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder};

#[derive(Debug, Clone, Parser)]
#[clap(rename_all = "kebab-case")]
struct Args {
    #[clap(short, long, env, default_value = "Shaders.toml")]
    manifest_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let args = Args::parse();
    let manifest = load_manifest(&args.manifest_path)?;

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = {
        let tx = tx.clone();

        notify::recommended_watcher(move |res| match res {
            Ok(_) => tx.send(()).unwrap(),
            Err(e) => log::error!("watch error: {:?}", e),
        })?
    };

    for shader in &manifest.shaders {
        watcher
            .watch(&shader.path_to_shader, notify::RecursiveMode::Recursive)?;
    }

    for dir in &manifest.dirs {
        watcher.watch(dir, notify::RecursiveMode::Recursive)?;
    }

    tx.send(()).unwrap();

    while let Ok(()) = rx.recv() {
        for shader in &manifest.shaders {
            build_shader(shader)?;
        }
    }

    Ok(())
}

fn load_manifest(path: impl AsRef<Path>) -> anyhow::Result<Manifest> {
    let manifest = fs::read_to_string(path)?;
    let manifest = toml::from_str(&manifest)?;

    Ok(manifest)
}

fn build_shader(shader_config: &ShaderConfig) -> anyhow::Result<()> {
    let build_result = SpirvBuilder::new(
        &shader_config.path_to_shader,
        "spirv-unknown-spv1.5",
    )
    .print_metadata(MetadataPrintout::None)
    .capability(Capability::Int8)
    .release(true)
    .build()?;

    if let Some(parent) = shader_config.path_to_copy_to.parent() {
        fs::create_dir_all(parent)?;
    }

    let spirv_module_path = shader_config.path_to_copy_to.with_extension("spv");
    let naga_module_path = shader_config.path_to_copy_to.with_extension("naga");

    fs_extra::file::copy(
        build_result.module.unwrap_single(),
        &spirv_module_path,
        &fs_extra::file::CopyOptions {
            overwrite: true,
            skip_exist: false,
            ..fs_extra::file::CopyOptions::default()
        },
    )?;

    transpile_to_naga(&spirv_module_path, &naga_module_path)?;

    Ok(())
}

fn transpile_to_naga(
    spirv_module_path: impl AsRef<Path>,
    naga_module_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let spirv_module_path = spirv_module_path.as_ref();
    let naga_module_path = naga_module_path.as_ref();
    let module = fs::read(spirv_module_path)?;

    let module =
        naga::front::spv::parse_u8_slice(&module, &Default::default())?;

    fs::write(naga_module_path, bson::to_vec(&module).unwrap()).unwrap();

    Ok(())
}
