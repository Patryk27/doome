use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Geometry {
    items: [Triangle; MAX_TRIANGLES],
    len: PadU32,
}

impl Geometry {
    pub fn get(&self, id: TriangleId) -> Triangle {
        self.items[id.get()]
    }

    pub fn len(&self) -> usize {
        self.len.value as _
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Geometry {
    pub fn push(&mut self, item: Triangle) -> TriangleId {
        let id = self.len.value as usize;

        self.items[id] = item;
        self.len += 1;

        TriangleId::new(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (TriangleId, Triangle)> + '_ {
        self.items[0..self.len()]
            .iter()
            .copied()
            .enumerate()
            .map(|(id, triangle)| (TriangleId::new(id), triangle))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Geometry {
    fn default() -> Self {
        Self::zeroed()
    }
}
