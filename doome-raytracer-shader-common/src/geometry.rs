use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Geometry {
    items: [Triangle; MAX_OBJECTS as _],
    len: PadU32,
}

impl Geometry {
    pub fn len(&self) -> u32 {
        self.len.value
    }

    pub fn get(&self, idx: u32) -> Triangle {
        self.items[idx as usize]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Geometry {
    pub fn push(&mut self, item: Triangle) -> u32 {
        let offset = self.len;

        self.items[self.len.value as usize] = item;
        self.len += 1;

        offset.value
    }

    pub fn write(&mut self, offset: u32, triangles: &[Triangle]) {
        let start = offset as usize;
        let end = start + triangles.len();

        self.items[start..end].copy_from_slice(triangles);

        if self.len.value < end as u32 {
            self.len.value = end as u32;
        }
    }

    pub fn get_mut(&mut self, idx: u32) -> &mut Triangle {
        &mut self.items[idx as usize]
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, Triangle)> + '_ {
        self.items[0..(self.len() as usize)]
            .iter()
            .enumerate()
            .map(|(triangle_id, triangle)| (triangle_id as _, *triangle))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Geometry {
    fn default() -> Self {
        Geometry::zeroed()
    }
}
