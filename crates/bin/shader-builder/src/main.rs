mod manifest;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use notify::Watcher;

use self::manifest::Manifest;

#[derive(Debug, Clone, Parser)]
#[clap(rename_all = "kebab-case")]
struct Args {
    #[clap(short, long, env, default_value = "Shaders.toml")]
    manifest_path: PathBuf,

    #[clap(short, long, env)]
    watch: bool,
}

fn main() -> Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let args = Args::parse();
    let manifest = Manifest::from_path(&args.manifest_path)?;

    if args.watch {
        watch(&manifest)
    } else {
        build(&manifest)
    }
}

fn watch(manifest: &Manifest) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = {
        let tx = tx.clone();

        notify::recommended_watcher(move |res| match res {
            Ok(_) => tx.send(()).unwrap(),
            Err(e) => panic!("Couldn't start file watcher: {:?}", e),
        })?
    };

    for shader in &manifest.shaders {
        watcher.watch(&shader.source_dir, notify::RecursiveMode::Recursive)?;
    }

    for dir in &manifest.extra_watch_dirs {
        watcher.watch(dir, notify::RecursiveMode::Recursive)?;
    }

    tx.send(()).unwrap();

    while let Ok(()) = rx.recv() {
        // Debounce
        loop {
            match rx.try_recv() {
                Ok(_) => (),
                Err(_) => break,
            }
        }

        _ = manifest.build();
    }

    Ok(())
}

fn build(manifest: &Manifest) -> Result<()> {
    manifest.build()
}
