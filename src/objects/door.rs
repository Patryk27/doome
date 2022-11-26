use crate::prelude::*;

pub struct Door {
    position: Vec2,
    rotation: Quat,
    key: Option<Key>,
}

impl Door {
    pub fn new() -> Self {
        Self {
            position: Default::default(),
            rotation: Default::default(),
            key: Default::default(),
        }
    }

    pub fn with_position(mut self, val: Vec2) -> Self {
        self.position = val;
        self
    }

    pub fn with_rotation(mut self, val: Quat) -> Self {
        self.rotation = val;
        self
    }

    pub fn with_key_opt(mut self, key: Option<Key>) -> Self {
        self.key = key;
        self
    }

    pub fn spawn(self, assets: &Assets, commands: &mut Commands) -> Entity {
        let model = assets.load_model("door");
        let position = vec3(self.position.x, 1.0, self.position.y);
        let rotation = self.rotation;

        let mut door = commands.spawn((
            model,
            Transform::from_translation(position)
                .with_rotation(rotation)
                .with_scale(vec3(0.5, 1.0, 1.0)),
            GeometryType::Dynamic,
            Material::default()
                .double_sided()
                .with_color(Color::hex(0xffffff))
                .with_uv_transparency(),
            Collider::line(vec2(0.0, -1.0), vec2(0.0, 1.0)),
        ));

        if let Some(key) = self.key {
            door.add_children(|commands| {
                let texture = assets.load_texture("picker.key");

                for sign in [-1.0, 1.0] {
                    let position =
                        position + rotation * vec3(0.05, 0.0, 0.05) * sign;

                    let position = position + vec3(0.0, 0.5, 0.0);

                    commands.spawn((
                        model,
                        Transform::from_translation(position)
                            .with_rotation(rotation)
                            .with_scale(vec3(0.2, 0.2, 0.2)),
                        GeometryType::Dynamic,
                        Material::default()
                            .double_sided()
                            .with_color(key.color())
                            .with_texture(texture)
                            .with_uv_transparency(),
                    ));
                }
            });

            door.insert(LockedDoor { key });
        }

        door.id()
    }
}

#[derive(Component)]
pub struct LockedDoor {
    pub key: Key,
}
