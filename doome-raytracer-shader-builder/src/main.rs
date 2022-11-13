use spirv_builder::SpirvBuilder;

fn main() {
    SpirvBuilder::new("../doome-raytracer-shader", "spirv-unknown-spv1.3")
        .release(false)
        .build()
        .unwrap();
}
