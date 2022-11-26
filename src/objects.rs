mod door;
mod flashlight;
mod gate;
mod key;
mod picker;

pub use self::door::*;
pub use self::flashlight::*;
pub use self::gate::*;
pub use self::key::*;
pub use self::picker::*;
use crate::prelude::*;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        let txt_unlock = app
            .world
            .spawn((
                Text::new("Press F to open the door").centered(),
                Transform::from_translation(vec3(0.5, 0.05, 0.0)),
                Visibility::invisible(),
            ))
            .id();

        app.insert_resource(LockedDoorsState { txt_unlock })
            .add_system(process_locked_doors);
    }
}

#[derive(Resource)]
struct LockedDoorsState {
    txt_unlock: Entity,
}

fn process_locked_doors(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    state: Res<LockedDoorsState>,
    player: Query<&Transform, With<Player>>,
    mut inventory: Query<&mut Inventory>,
    doors: Query<(Entity, &Transform, &LockedDoor)>,
    mut visibilities: Query<&mut Visibility>,
) {
    visibilities.get_mut(state.txt_unlock).unwrap().is_visible = false;

    let Ok(player_xform) = player.get_single() else { return };
    let Ok(mut inventory) = inventory.get_single_mut() else { return };

    // -----

    let mut nearby_door = None;

    for (door_entity, door_xform, door) in doors.iter() {
        let distance = player_xform
            .translation
            .xz()
            .distance(door_xform.translation.xz());

        if distance < 3.0 && inventory.has_key(&door.key) {
            nearby_door = Some((door_entity, door));
            break;
        }
    }

    // -----

    let Some((door_entity, door)) = nearby_door else { return };

    visibilities.get_mut(state.txt_unlock).unwrap().is_visible = true;

    if keys.pressed(KeyCode::F) {
        inventory.remove_key(&door.key);
        commands.entity(door_entity).despawn_recursive();
    }
}
