use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GeometryIndex {
    data: [Vec4; 4096],
}

impl GeometryIndex {
    pub fn read(&self, ptr: usize) -> Vec4 {
        self.data[ptr]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl GeometryIndex {
    pub fn new(data: [Vec4; 4096]) -> Self {
        Self { data }
    }
}
