use bevy::prelude::*;

use crate::player::Player;

pub struct BillboardPlugin;

#[derive(Component)]
pub struct Billboard;

impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_to_stage(CoreStage::PostUpdate, align_billboards);
    }
}

fn align_billboards(
    player: Query<(&Player, &Transform), With<Player>>,
    mut billboards: Query<(&Billboard, &mut Transform), Without<Player>>,
) {
    let (_, player_transform) = player.single();

    for (_billboard, mut transform) in billboards.iter_mut() {
        transform.look_at(player_transform.translation, Vec3::Y);
    }
}
