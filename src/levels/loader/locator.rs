use std::f32::consts::PI;

use super::*;

#[derive(Clone, Debug, Default)]
pub struct LevelLocator {
    objects: HashMap<String, Object>,
    entities: HashMap<String, Entity>,
}

impl LevelLocator {
    pub(super) fn add(&mut self, name: String, x: i32, y: i32, w: i32, h: i32) {
        self.objects.insert(name, Object { x, y, w, h });
    }

    pub(super) fn spawn(&mut self, lvl: &mut LevelBuilder) {
        for name in self.objects.keys() {
            let pname = name.strip_prefix("$:");

            if name.starts_with("$:door") {
                let rot = if name.contains("door.h") {
                    Quat::from_rotation_y(0.0)
                } else if name.contains("door.v") {
                    Quat::from_rotation_y(PI / 2.0)
                } else {
                    panic!("Invalid door specification: {}", name);
                };

                let entity = Door::spawn(
                    lvl.assets(),
                    lvl.commands(),
                    self.world_point(name),
                    rot,
                );

                self.entities.insert(pname.unwrap().to_owned(), entity);
            }

            if name.starts_with("$:gate") {
                let (rot, ext) = if name.contains(".north") {
                    (Quat::from_rotation_y(3.0 * PI / 2.0), vec3(0.0, 0.0, 0.0))
                } else if name.contains(".east") {
                    (Quat::from_rotation_y(PI), vec3(0.0, 0.0, 0.5))
                } else if name.contains(".west") {
                    (Quat::from_rotation_y(0.0), vec3(0.0, 0.0, -0.5))
                } else {
                    (Quat::from_rotation_y(PI / 2.0), vec3(0.0, 0.0, 0.0))
                };

                Gate::spawn(
                    lvl.assets(),
                    lvl.commands(),
                    self.world_point(name) + ext,
                    rot,
                );
            }

            if name.starts_with("$:heart") {
                Heart::spawn(
                    lvl.assets(),
                    lvl.commands(),
                    self.world_point(name),
                );
            }

            if name.starts_with("$:zone") {
                let zone = self.rect(name);
                let zone_name = name.strip_prefix("$:").unwrap();

                lvl.zone(
                    zone_name,
                    zone.0 as f32,
                    zone.1 as f32,
                    (zone.0 + zone.2) as f32,
                    (zone.1 + zone.3) as f32,
                )
            }
        }
    }

    pub fn point(&self, name: &str) -> (i32, i32) {
        let object = self.get(name);

        if object.w != 0 || object.h != 0 {
            panic!("Map object `{}` is not a point", name);
        }

        (object.x, object.y)
    }

    pub fn world_point(&self, name: &str) -> Vec3 {
        let (x, z) = self.point(name);

        vec3(x as f32, 0.0, z as f32)
    }

    pub fn rect(&self, name: &str) -> (i32, i32, i32, i32) {
        let object = self.get(name);

        if object.w == 0 || object.h == 0 {
            panic!("Map object `{}` is not a rectange", name);
        }

        (object.x, object.y, object.w, object.h)
    }

    pub fn entity(&self, name: &str) -> Entity {
        self.entities.get(name).cloned().unwrap_or_else(|| {
            panic!("Map object `{}` has no associated entity", name)
        })
    }

    fn get(&self, name: &str) -> Object {
        self.objects
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("Map object `{}` does not exist", name))
    }
}

#[derive(Clone, Copy, Debug)]
struct Object {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}
