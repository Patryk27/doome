use bevy::prelude::Component;

#[derive(Clone, Copy, Component)]
pub struct InteractableHighlight;

#[derive(Clone, Copy, Component)]
pub struct FollowPlayerAbove {
    pub offset: f32,
}
