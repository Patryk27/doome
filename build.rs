use spirv_builder::SpirvBuilder;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let result = SpirvBuilder::new("raytracer-shader", "spirv-unknown-spv1.3")
        .release(true)
        .build()
        .unwrap();

    let path = result.module.unwrap_single();

    println!("cargo:rustc-env=DOOME_RAYTRACER_SHADER={}", path.display());
}
