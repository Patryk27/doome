use crate::*;

/// Maps triangle vertices into UVs.
///
/// # Memory layout
///
/// One triangle's UVs take `3 [vertices] * 2 [f32 per vertice]` = `6 [f32]`,
/// which means that we can store two triangles worth of UVs in three `Vec4`,
/// giving us:
///
/// ```text
/// mapping #0              mapping #1
/// =====================   =====================
/// uvs[0]          uvs[1]          uvs[2]
/// -------------   -------------   -------------
/// x   y   z   w   x   y   z   w   x   y   z   w
/// .....   .....   .....   .....   .....   .....
/// uv0     uv1     uv2     uv0     uv1     uv2
/// ```
///
/// (`uvs` here standing for either `static_uvs` or `dynamic_uvs`.)
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TriangleMappings {
    static_uvs: [Vec4; 3 * MAX_STATIC_TRIANGLES / 2],
    dynamic_uvs: [Vec4; 3 * MAX_DYNAMIC_TRIANGLES / 2],
}

impl TriangleMappings {
    pub fn get(&self, id: TriangleId<AnyTriangle>) -> TriangleMapping {
        match id.unpack() {
            (AnyTriangle::Static, id) => Self::get_ex(&self.static_uvs, id),
            (AnyTriangle::Dynamic, id) => Self::get_ex(&self.dynamic_uvs, id),
        }
    }

    fn get_ex<const N: usize>(uvs: &[Vec4; N], id: usize) -> TriangleMapping {
        if id % 2 == 0 {
            let ptr = 3 * id;

            TriangleMapping {
                uv0: uvs[ptr].xy(),
                uv1: uvs[ptr].zw(),
                uv2: uvs[ptr + 1].xy(),
            }
        } else {
            let ptr = 3 * (id - 1) + 1;

            TriangleMapping {
                uv0: uvs[ptr].zw(),
                uv1: uvs[ptr + 1].xy(),
                uv2: uvs[ptr + 1].zw(),
            }
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl TriangleMappings {
    pub fn set(&mut self, id: TriangleId<AnyTriangle>, item: TriangleMapping) {
        match id.unpack() {
            (AnyTriangle::Static, id) => {
                Self::set_ex(&mut self.static_uvs, id, item)
            }
            (AnyTriangle::Dynamic, id) => {
                Self::set_ex(&mut self.dynamic_uvs, id, item)
            }
        }
    }

    fn set_ex(
        uvs: &mut [Vec4],
        id: usize,
        TriangleMapping { uv0, uv1, uv2 }: TriangleMapping,
    ) {
        if id % 2 == 0 {
            let ptr = &mut uvs[3 * id..][..2];

            ptr[0].x = uv0.x;
            ptr[0].y = uv0.y;
            ptr[0].z = uv1.x;
            ptr[0].w = uv1.y;
            ptr[1].x = uv2.x;
            ptr[1].y = uv2.y;
        } else {
            let ptr = &mut uvs[3 * (id - 1) + 1..][..2];

            ptr[0].z = uv0.x;
            ptr[0].w = uv0.y;
            ptr[1].x = uv1.x;
            ptr[1].y = uv1.y;
            ptr[1].z = uv2.x;
            ptr[1].w = uv2.y;
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for TriangleMappings {
    fn default() -> Self {
        Self::zeroed()
    }
}
