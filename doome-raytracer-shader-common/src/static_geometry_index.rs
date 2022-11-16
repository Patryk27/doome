use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct StaticGeometryIndex {
    data: [Vec4; 4096],
}

impl StaticGeometryIndex {
    pub fn read(&self, ptr: usize) -> Vec4 {
        self.data[ptr]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl StaticGeometryIndex {
    pub fn new(data: [Vec4; 4096]) -> Self {
        Self { data }
    }
}