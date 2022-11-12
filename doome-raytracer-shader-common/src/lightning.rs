use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Lightning {
    items: [Light; MAX_LIGHTS as _],
    len: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
}

impl Lightning {
    pub fn push(&mut self, item: Light) {
        self.items[self.len as usize] = item;
        self.len += 1;
    }

    pub fn get(&self, idx: u32) -> Light {
        self.items[idx as usize]
    }

    pub fn get_mut(&mut self, idx: u32) -> &mut Light {
        &mut self.items[idx as usize]
    }

    pub fn len(&self) -> u32 {
        self.len
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Lightning {
    fn default() -> Self {
        Lightning::zeroed()
    }
}
