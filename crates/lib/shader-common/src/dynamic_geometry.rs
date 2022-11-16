use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct DynamicGeometry {
    items: [Triangle; MAX_DYNAMIC_TRIANGLES],
    len: PadU32,
}

impl DynamicGeometry {
    pub fn get(&self, id: TriangleId<DynamicTriangle>) -> Triangle {
        self.items[id.id()]
    }

    pub fn len(&self) -> usize {
        self.len.value as _
    }
}

#[cfg(not(target_arch = "spirv"))]
impl DynamicGeometry {
    pub fn push(&mut self, item: Triangle) -> TriangleId<DynamicTriangle> {
        let id = self.len.value as usize;

        self.items[id] = item;
        self.len += 1;

        TriangleId::new_dynamic(id)
    }

    pub fn get_mut(
        &mut self,
        id: TriangleId<DynamicTriangle>,
    ) -> &mut Triangle {
        &mut self.items[id.id()]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for DynamicGeometry {
    fn default() -> Self {
        Self::zeroed()
    }
}
