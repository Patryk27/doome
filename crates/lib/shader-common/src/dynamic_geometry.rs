use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct DynamicGeometry {
    items: [Triangle; MAX_DYNAMIC_TRIANGLES],
}

impl DynamicGeometry {
    pub fn get(&self, id: TriangleId<DynamicTriangle>) -> Triangle {
        self.items[id.get()]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl DynamicGeometry {
    pub fn set(&mut self, id: TriangleId<DynamicTriangle>, item: Triangle) {
        self.items[id.get()] = item;
    }

    pub fn remove(&mut self, id: TriangleId<DynamicTriangle>) {
        self.set(id, Default::default());
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for DynamicGeometry {
    fn default() -> Self {
        Self::zeroed()
    }
}
