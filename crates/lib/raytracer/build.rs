use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let profile = env::var("PROFILE").unwrap();

    println!("cargo:rustc-env=PROFILE={}", profile);

    let mut dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let ok = dir.ends_with("out")
        && dir.pop()
        && dir.pop()
        && dir.ends_with("build")
        && dir.pop()
        && dir.ends_with(profile)
        && dir.pop();

    assert!(ok);

    let dir = dir.join("shader-builder");
    let dir = dir.display().to_string();

    let mut cargo = std::process::Command::new("cargo")
        .args([
            "run",
            "--release",
            "-p",
            "shader-builder",
            "--target-dir",
            &dir,
            "--",
            "--path-to-shader",
            "../../shaders/raytracer",
        ])
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .spawn()
        .unwrap();

    if !cargo.wait().unwrap().success() {
        panic!("Couldn't compile shader");
    }
}
