use bevy::prelude::*;

pub struct NavigationPlugin;

#[derive(Component)]
pub struct NavObstacle;

impl Plugin for NavigationPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {
        // do nothing
    }
}
