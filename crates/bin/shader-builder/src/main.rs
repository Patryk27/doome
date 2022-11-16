use std::path::PathBuf;

use clap::Parser;
use spirv_builder::{Capability, SpirvBuilder};

#[derive(Debug, Clone, Parser)]
#[clap(rename_all = "kebab-case")]
struct Args {
    #[clap(short, long, env)]
    path_to_shader: PathBuf,

    #[clap(short, long, env, default_value = "spirv-unknown-spv1.3")]
    shader_target: String,

    #[clap(short, long)]
    copy_to: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let args = Args::parse();

    let build_result =
        SpirvBuilder::new(args.path_to_shader, args.shader_target)
            .capability(Capability::Int8)
            .release(true)
            .build()?;

    if let Some(copy_to) = args.copy_to {
        if let Some(parent) = copy_to.parent() {
            std::fs::create_dir_all(parent)?;
        }

        fs_extra::file::move_file(
            build_result.module.unwrap_single(),
            copy_to,
            &fs_extra::file::CopyOptions {
                overwrite: true,
                skip_exist: false,
                ..fs_extra::file::CopyOptions::default()
            },
        )?;
    }

    println!("{:#?}", build_result);

    Ok(())
}
