use bevy::prelude::Resource;

#[derive(Resource)]
pub struct RenderingOptions {
    pub sse_enabled: bool,
    pub debug_pass_enabled: bool,
}
