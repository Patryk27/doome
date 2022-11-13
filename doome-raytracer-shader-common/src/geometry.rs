use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Geometry {
    items: [Triangle; MAX_OBJECTS as _],
    len: PadU32,
}

impl Geometry {
    pub fn push(&mut self, item: Triangle) {
        self.items[self.len.value as usize] = item;
        self.len += 1;
    }

    pub fn len(&self) -> u32 {
        self.len.value
    }

    pub fn get(&self, idx: u32) -> Triangle {
        self.items[idx as usize]
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn get_mut(&mut self, idx: u32) -> &mut Triangle {
        &mut self.items[idx as usize]
    }

    #[cfg(not(target_arch = "spirv"))]
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
