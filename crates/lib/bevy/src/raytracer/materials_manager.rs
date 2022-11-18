use std::collections::HashSet;

use bevy::prelude::Entity;
use doome_raytracer as rt;
use glam::vec3;

pub struct MaterialsManager {
    materials: Box<rt::Materials>,
    owners: Vec<HashSet<Entity>>,
}

impl MaterialsManager {
    pub fn alloc(
        &mut self,
        entity: Entity,
        material: rt::Material,
    ) -> rt::MaterialId {
        for id in 0..rt::MAX_MATERIALS {
            let id = rt::MaterialId::new(id);

            if self.materials.get(id) == material {
                self.owners[id.get()].insert(entity);
                return id;
            }
        }

        for id in 0..rt::MAX_MATERIALS {
            if self.owners[id].contains(&entity) {
                if self.owners[id].len() == 1 {
                    let id = rt::MaterialId::new(id);

                    self.materials.set(id, material);

                    return id;
                } else {
                    self.owners[id].remove(&entity);
                }
            }
        }

        log::trace!("Allocating: {:?} (material={:?})", entity, material);

        let id = (0..rt::MAX_MATERIALS)
            .map(rt::MaterialId::new)
            .find(|id| self.owners[id.get()].is_empty())
            .expect("Tried to allocate too many materials at once");

        self.materials.set(id, material);
        self.owners[id.get()].insert(entity);

        id
    }

    pub fn free(&mut self, entity: Entity) {
        log::trace!("Freeing: {:?}", entity);

        for id in 0..rt::MAX_MATERIALS {
            self.owners[id].remove(&entity);
        }
    }

    pub fn dummy(&mut self, entity: Entity) -> rt::MaterialId {
        self.alloc(
            entity,
            rt::Material::default().with_color(vec3(1.0, 1.0, 1.0)),
        )
    }

    pub fn inner(&self) -> &rt::Materials {
        &self.materials
    }
}

impl Default for MaterialsManager {
    fn default() -> Self {
        Self {
            materials: Default::default(),
            owners: vec![Default::default(); rt::MAX_MATERIALS],
        }
    }
}
