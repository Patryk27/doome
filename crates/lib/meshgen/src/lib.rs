use doome_shader_common::Triangle;

pub struct GeneratedMesh {
    pub triangles: Vec<Triangle>,
}

/// Builds a flat wall that's 4 units tall
pub fn build_wall(
    x1: i32,
    z1: i32,
    x2: i32,
    z2: i32,
    rot: u8,
) -> GeneratedMesh {
    todo!()
}
