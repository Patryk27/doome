use doome_raytracer as rt;
use glam::vec3;

#[derive(Default)]
pub struct MaterialsManager {
    library: rt::Materials,
    allocated: [bool; rt::MAX_MATERIALS],
}

// TODO release
impl MaterialsManager {
    pub fn allocate(&mut self, mat: rt::Material) -> rt::MaterialId {
        if let Some(id) = self.lookup(mat) {
            self.allocated[id.get()] = true;
            return id;
        }

        let id = self
            .find_slot()
            .expect("Tried to allocate too many materials at once");

        self.library.set(id, mat);
        self.allocated[id.get()] = true;

        id
    }

    pub fn dummy(&mut self) -> rt::MaterialId {
        self.allocate(rt::Material::default().with_color(vec3(1.0, 1.0, 1.0)))
    }

    pub fn library(&self) -> &rt::Materials {
        &self.library
    }

    fn lookup(&self, mat: rt::Material) -> Option<rt::MaterialId> {
        for id in 0..rt::MAX_MATERIALS {
            let id = rt::MaterialId::new(id);

            if self.library.get(id) == mat {
                return Some(id);
            }
        }

        None
    }

    fn find_slot(&self) -> Option<rt::MaterialId> {
        for id in 0..rt::MAX_MATERIALS {
            if !self.allocated[id] {
                return Some(rt::MaterialId::new(id));
            }
        }

        None
    }
}
