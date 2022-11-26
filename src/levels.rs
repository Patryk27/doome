use bevy::prelude::*;

pub use self::builder::*;
pub use self::coordinator::*;
pub use self::loader::*;
pub use self::zone::*;

pub mod level1;
pub mod level2;

mod builder;
mod coordinator;
mod loader;
mod zone;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GotoLevel>()
            .add_event::<LevelGameplayEvent>()
            .add_startup_system(LevelsCoordinator::init)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                ordered_systems! {
                    LevelsCoordinator::unload
                    => level1::init
                    => level2::init
                },
            )
            .add_system(level1::process.after(level1::init))
            .add_system(level2::process.after(level2::init))
            .add_system(LevelsCoordinator::process_zones);
    }
}

#[derive(Component)]
pub struct GcAfterLevelUnloaded;
