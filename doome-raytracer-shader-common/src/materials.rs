use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Materials {
    items: [Material; MAX_MATERIALS as _],
    len: PadU32,
}

impl Materials {
    pub fn push(&mut self, item: Material) -> MaterialId {
        let id = self.len.value;

        self.items[self.len.value as usize] = item;
        self.len += 1;

        MaterialId::new(id as _)
    }

    pub fn len(&self) -> u32 {
        self.len.value
    }

    pub fn get(&self, id: MaterialId) -> Material {
        self.items[id.get() as usize]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Materials {
    pub fn get_mut(&mut self, id: MaterialId) -> &mut Material {
        &mut self.items[id.get() as usize]
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Materials {
    fn default() -> Self {
        Self::zeroed()
    }
}
