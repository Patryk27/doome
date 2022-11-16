use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct StaticGeometryMapping {
    items: [TriangleMapping; MAX_STATIC_TRIANGLES],
}

impl StaticGeometryMapping {
    pub fn get(&self, id: TriangleId<StaticTriangle>) -> TriangleMapping {
        self.items[id.id()]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl StaticGeometryMapping {
    pub fn set(
        &mut self,
        id: TriangleId<StaticTriangle>,
        item: TriangleMapping,
    ) {
        self.items[id.id()] = item;
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for StaticGeometryMapping {
    fn default() -> Self {
        Self::zeroed()
    }
}
