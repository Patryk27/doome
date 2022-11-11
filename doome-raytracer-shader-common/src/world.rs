use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct World {
    objects: [Object; MAX_OBJECTS as _],
    objects_count: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
}

impl World {
    pub fn objects(&self) -> &[Object; MAX_OBJECTS as _] {
        &self.objects
    }

    pub fn objects_count(&self) -> u32 {
        self.objects_count
    }

    pub fn push_object(&mut self, object: Object) {
        self.objects[self.objects_count as usize] = object;
        self.objects_count += 1;
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for World {
    fn default() -> Self {
        World::zeroed()
    }
}
