use spirv_builder::{Capability, SpirvBuilder};

fn main() {
    SpirvBuilder::new("../doome-raytracer-shader", "spirv-unknown-spv1.3")
        .release(true)
        .build()
        .unwrap();
}
