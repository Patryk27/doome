use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct StaticGeometry {
    items: [Triangle; MAX_STATIC_TRIANGLES],
    len: PadU32,
}

impl StaticGeometry {
    pub fn get(&self, id: TriangleId<StaticTriangle>) -> Triangle {
        self.items[id.id()]
    }

    pub fn len(&self) -> usize {
        self.len.value as _
    }
}

#[cfg(not(target_arch = "spirv"))]
impl StaticGeometry {
    pub fn push(&mut self, item: Triangle) -> TriangleId<StaticTriangle> {
        let id = self.len.value as usize;

        self.items[id] = item;
        self.len += 1;

        TriangleId::new_static(id)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (TriangleId<StaticTriangle>, Triangle)> + '_ {
        self.items[0..self.len()]
            .iter()
            .copied()
            .enumerate()
            .map(|(id, triangle)| (TriangleId::new_static(id), triangle))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for StaticGeometry {
    fn default() -> Self {
        Self::zeroed()
    }
}
