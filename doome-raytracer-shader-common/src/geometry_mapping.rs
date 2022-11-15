use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GeometryMapping {
    items: [TriangleMapping; MAX_TRIANGLES],
}

impl GeometryMapping {
    pub fn get(&self, id: TriangleId) -> TriangleMapping {
        self.items[id.get()]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl GeometryMapping {
    pub fn set(&mut self, id: TriangleId, item: TriangleMapping) {
        self.items[id.get()] = item;
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for GeometryMapping {
    fn default() -> Self {
        Self::zeroed()
    }
}
