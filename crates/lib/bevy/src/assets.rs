use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Assets {
    pub dir: include_dir::Dir<'static>,
}
