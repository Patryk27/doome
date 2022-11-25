use std::f32::consts::PI;

use super::*;

#[derive(Clone, Debug, Default)]
pub struct LevelLocator {
    objects: HashMap<String, Vec<Object>>,
    entities: HashMap<String, Entity>,
    tags: HashMap<String, Vec3>,
}

impl LevelLocator {
    pub(super) fn add(&mut self, name: String, x: i32, y: i32, w: i32, h: i32) {
        self.objects
            .entry(name)
            .or_default()
            .push(Object { x, y, w, h });
    }

    pub(super) fn spawn(
        &mut self,
        imap: &indexed::Map<'_>,
        lvl: &mut LevelBuilder,
    ) {
        let has_wall_at = |x, y| imap.get(x, y).any(|tile| tile.is_wall());

        let objects = self
            .objects
            .iter()
            .flat_map(|(k, vs)| vs.iter().map(move |v| (k, v)));

        for (obj_name, obj) in objects {
            if let Some(name) = obj_name.strip_prefix("door:") {
                let rot = if has_wall_at(obj.x - 1, obj.y)
                    || has_wall_at(obj.x + 1, obj.y)
                {
                    Quat::from_rotation_y(2.0 * PI / 2.0)
                } else {
                    Quat::from_rotation_y(-PI / 2.0)
                };

                let entity = Door::spawn(
                    lvl.assets(),
                    lvl.commands(),
                    obj.location(),
                    rot,
                );

                self.entities.insert(name.to_owned(), entity);
                continue;
            }

            if obj_name == "gate" {
                let rot = if has_wall_at(obj.x - 1, obj.y)
                    || has_wall_at(obj.x + 1, obj.y)
                {
                    Quat::from_rotation_y(PI / 2.0)
                } else {
                    Default::default()
                };

                Gate::spawn(lvl.assets(), lvl.commands(), obj.location(), rot);
                continue;
            }

            if obj_name == "heart" {
                Heart::spawn(lvl.assets(), lvl.commands(), obj.location());
                continue;
            }

            if let Some(name) = obj_name.strip_prefix("tag:") {
                if self.tags.insert(name.to_owned(), obj.location()).is_some() {
                    panic!("Map contains tag defined multiple times: {}", name);
                }

                continue;
            }

            if let Some(name) = obj_name.strip_prefix("zone:") {
                lvl.zone(
                    name,
                    obj.x as f32,
                    obj.y as f32,
                    (obj.x + obj.w) as f32,
                    (obj.y + obj.h) as f32,
                );

                continue;
            }

            panic!("Map contains unrecognized object: {}", obj_name);
        }
    }

    pub fn entity(&self, name: &str) -> Entity {
        self.entities.get(name).cloned().unwrap_or_else(|| {
            panic!("Map contains no entity called `{}`", name)
        })
    }

    pub fn tag(&self, name: &str) -> Vec3 {
        self.tags.get(name).cloned().unwrap_or_else(|| {
            panic!("Map contains no tag called `{}`", name);
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Object {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Object {
    fn location(self) -> Vec3 {
        vec3(self.x as f32, 0.0, self.y as f32)
    }
}
