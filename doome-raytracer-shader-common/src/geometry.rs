use crate::padded::PadU32;
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

    pub fn get_mut(&mut self, idx: u32) -> &mut Triangle {
        &mut self.items[idx as usize]
    }

    pub fn any_obstacle_between(&self, a: Vec3, b: Vec3) -> bool {
        let mut idx = 0;
        let len = (b - a).length();

        let ray = Ray {
            origin: a,
            direction: (b - a),
        };

        while idx < self.len() {
            if self.items[idx as usize].hit(ray).t < len {
                return true;
            }

            idx += 1;
        }

        false
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Geometry {
    fn default() -> Self {
        Geometry::zeroed()
    }
}
