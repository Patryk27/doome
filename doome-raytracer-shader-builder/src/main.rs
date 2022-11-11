use spirv_builder::{Capability, SpirvBuilder};

fn main() {
    SpirvBuilder::new("../doome-raytracer-shader", "spirv-unknown-spv1.3")
        .capability(Capability::Int8)
        .release(true)
        .build()
        .unwrap();
}
