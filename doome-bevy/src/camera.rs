use bevy::prelude::Plugin;

pub struct CameraPlugin;

pub struct CameraState {}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}
