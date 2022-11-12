use crate::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Geometry {
    items: [Triangle; MAX_OBJECTS as _],
    len: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
}

impl Geometry {
    pub fn push(&mut self, item: Triangle) {
        self.items[self.len as usize] = item;
        self.len += 1;
    }

    pub fn any_obstacle_between(&self, a: Vec3, b: Vec3) -> bool {
        let mut idx = 0;
        let len = (b - a).length().abs();

        let ray = Ray {
            origin: a,
            direction: (b - a),
        };

        while idx < self.len() {
            let hit = self.items[idx as usize].hit(ray);

            // TODO
            if hit.is_some() {
                if hit.t.abs() < len {
                    return true;
                }
            }

            idx += 1;
        }

        false
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn all(&self) -> &[Triangle; MAX_OBJECTS as _] {
        &self.items
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Default for Geometry {
    fn default() -> Self {
        Geometry::zeroed()
    }
}
