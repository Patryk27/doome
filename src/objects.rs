mod column;
mod door;
mod flashlight;
mod gate;
mod key;
mod picker;
mod torch;

pub use self::column::*;
pub use self::door::*;
pub use self::flashlight::*;
pub use self::gate::*;
pub use self::key::*;
pub use self::picker::*;
pub use self::torch::*;
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
            .add_system(LockedDoor::process)
            .add_system(Torch::process);
    }
}

#[derive(Resource)]
struct LockedDoorsState {
    txt_unlock: Entity,
}
