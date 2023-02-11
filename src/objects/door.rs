use super::LockedDoorsState;
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

    pub fn spawn(self, assets: &DoomeAssets, commands: &mut Commands) -> Entity {
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
                .with_uv_transparency()
                .without_casting_shadows(),
            Collider::line(vec2(-1.0, 0.0), vec2(1.0, 0.0)),
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
                            .with_uv_transparency()
                            .without_casting_shadows(),
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

impl LockedDoor {
    pub(super) fn process(
        mut commands: Commands,
        keys: Res<Input<KeyCode>>,
        state: Res<LockedDoorsState>,
        player: Query<&Transform, With<Player>>,
        mut inventory: Query<&mut Inventory>,
        doors: Query<(Entity, &Transform, &LockedDoor, &Children)>,
        mut visibilities: Query<&mut Visibility>,
        mut level_tx: EventWriter<LevelGameplayEvent>,
    ) {
        visibilities.get_mut(state.txt_unlock).unwrap().is_visible = false;

        let Ok(player_xform) = player.get_single() else { return };
        let Ok(mut inventory) = inventory.get_single_mut() else { return };

        // -----

        let mut nearby_door = None;

        for (door_entity, door_xform, door, door_children) in doors.iter() {
            let distance = player_xform
                .translation
                .xz()
                .distance(door_xform.translation.xz());

            if distance < 3.0 && inventory.has_key(&door.key) {
                nearby_door = Some((door_entity, door, door_children));
                break;
            }
        }

        // -----

        let Some((door_entity, door, door_children)) = nearby_door else { return };

        visibilities.get_mut(state.txt_unlock).unwrap().is_visible = true;

        if keys.pressed(KeyCode::F) {
            inventory.remove_key(&door.key);

            commands
                .entity(door_entity)
                .remove::<Collider>()
                .insert(Fade::fade_out(1.25));

            for door_child in door_children {
                commands.entity(*door_child).insert(Fade::fade_out(1.25));
            }

            level_tx.send(LevelGameplayEvent::DoorOpened(
                door.key.name().to_owned(),
            ));
        }
    }
}
