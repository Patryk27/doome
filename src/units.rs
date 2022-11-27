mod doome;
mod moth_monster;

use bevy::prelude::*;

pub use self::doome::*;
pub use self::moth_monster::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Doome::animate);
    }
}
