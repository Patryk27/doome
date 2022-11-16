use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct DynamicGeometryMapping {
    items: [TriangleMapping; MAX_DYNAMIC_TRIANGLES],
}

impl DynamicGeometryMapping {
    pub fn get(&self, id: TriangleId<DynamicTriangle>) -> TriangleMapping {
        self.items[id.id()]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl DynamicGeometryMapping {
    pub fn set(
        &mut self,
        id: TriangleId<DynamicTriangle>,
        item: TriangleMapping,
    ) {
        self.items[id.id()] = item;
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for DynamicGeometryMapping {
    fn default() -> Self {
        Self::zeroed()
    }
}
