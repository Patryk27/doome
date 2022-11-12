use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Lights {
    items: [Light; MAX_LIGHTS as _],
    len: PadU32,
}

impl Lights {
    pub fn push(&mut self, item: Light) {
        self.items[self.len.value as usize] = item;
        self.len += 1;
    }

    pub fn get(&self, idx: u32) -> Light {
        self.items[idx as usize]
    }

    #[cfg(not(target_arch = "spirv"))]
    pub fn get_mut(&mut self, idx: u32) -> &mut Light {
        &mut self.items[idx as usize]
    }

    pub fn len(&self) -> u32 {
        self.len.value
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Lights {
    fn default() -> Self {
        Lights::zeroed()
    }
}
