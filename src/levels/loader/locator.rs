use std::f32::consts::PI;

use itertools::Itertools;

use super::*;

#[derive(Clone, Debug, Default)]
pub struct LevelLocator {
    objects: HashMap<String, Vec<Object>>,
    doors: HashMap<String, Entity>,
    keys: HashMap<String, Entity>,
    tags: HashMap<String, Vec2>,
    torches: HashMap<String, Entity>,
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
            if let Some(spec) = obj_name.strip_prefix("door:") {
                let (name, key) = if spec.contains(",") {
                    let (name, color) =
                        spec.split(",").collect_tuple().unwrap();

                    let color = color.strip_prefix("0x").unwrap();
                    let color = u32::from_str_radix(color, 16).unwrap();
                    let color = Color::hex(color);

                    let key = Key::new(name, color);

                    (name, Some(key))
                } else {
                    (spec, None)
                };

                let rot = if has_wall_at(obj.x - 1, obj.y)
                    || has_wall_at(obj.x + 1, obj.y)
                {
                    Quat::from_rotation_y(2.0 * PI / 2.0)
                } else {
                    Quat::from_rotation_y(-PI / 2.0)
                };

                let entity = Door::new()
                    .with_position(obj.position())
                    .with_rotation(rot)
                    .with_key_opt(key)
                    .spawn(lvl.assets(), lvl.commands());

                self.doors.insert(name.to_owned(), entity);
                continue;
            }

            if obj_name == "gate" {
                let rot = if has_wall_at(obj.x - 1, obj.y) {
                    Quat::from_rotation_y(PI / 2.0)
                } else {
                    Default::default()
                };

                Gate::spawn(lvl.assets(), lvl.commands(), obj.position(), rot);
                continue;
            }

            if obj_name == "heart" {
                Picker::heart()
                    .with_position(obj.position())
                    .spawn(lvl.assets(), lvl.commands());

                continue;
            }

            if let Some(spec) = obj_name.strip_prefix("key:") {
                let (name, color) =
                    spec.split(",").collect_tuple().unwrap_or_else(|| {
                        panic!("Map contains invalid key definition: {}", spec);
                    });

                let color = color.strip_prefix("0x").unwrap();
                let color = u32::from_str_radix(color, 16).unwrap();
                let color = Color::hex(color);

                let entity = Picker::key(Key::new(name, color))
                    .with_position(obj.position())
                    .spawn(lvl.assets(), lvl.commands());

                self.keys.insert(name.to_owned(), entity);
                continue;
            }

            if let Some(name) = obj_name.strip_prefix("tag:") {
                if self.tags.insert(name.to_owned(), obj.position()).is_some() {
                    panic!("Map contains tag defined multiple times: {}", name);
                }

                continue;
            }

            if let Some(spec) = obj_name.strip_prefix("torch:") {
                let mut opts = spec.split(",");

                let name = opts.next().unwrap();
                let mut active = true;
                let mut force_active_texture = false;

                for opt in opts {
                    match opt {
                        "off" => {
                            active = false;
                        }
                        "force-active-texture" => {
                            force_active_texture = true;
                        }
                        _ => {
                            panic!(
                                "Map contains invalid torch definition: {}",
                                name
                            );
                        }
                    }
                }

                let rot = if has_wall_at(obj.x - 1, obj.y) {
                    Default::default()
                } else if has_wall_at(obj.x + 1, obj.y) {
                    Quat::from_rotation_y(PI)
                } else if has_wall_at(obj.x, obj.y - 1) {
                    Quat::from_rotation_y(-PI / 2.0)
                } else {
                    Default::default()
                };

                let entity = Torch::new()
                    .with_active(active)
                    .with_force_active_texture(force_active_texture)
                    .with_position(obj.position())
                    .with_rotation(rot)
                    .spawn(lvl.assets(), lvl.commands());

                if self.torches.insert(name.to_owned(), entity).is_some() {
                    panic!(
                        "Map contains torch defined multiple times: {}",
                        name
                    );
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

    pub fn door(&self, name: impl AsRef<str>) -> Entity {
        let name = name.as_ref();

        self.doors
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("Map contains no door called `{}`", name))
    }

    pub fn key(&self, name: impl AsRef<str>) -> Entity {
        let name = name.as_ref();

        self.keys
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("Map contains no key called `{}`", name))
    }

    pub fn tag(&self, name: impl AsRef<str>) -> Vec3 {
        let name = name.as_ref();

        self.tags
            .get(name)
            .map(|pos| vec3(pos.x, 0.0, pos.y))
            .unwrap_or_else(|| {
                panic!("Map contains no tag called `{}`", name);
            })
    }

    pub fn torch(&self, name: impl AsRef<str>) -> Entity {
        let name = name.as_ref();

        self.torches.get(name).cloned().unwrap_or_else(|| {
            panic!("Map contains no torch called `{}`", name)
        })
    }

    pub fn torches(&self) -> impl Iterator<Item = (&str, Entity)> + '_ {
        self.torches.iter().map(|(k, v)| (k.as_str(), *v))
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
    fn position(self) -> Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
}
