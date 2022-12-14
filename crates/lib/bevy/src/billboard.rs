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
        let mut target_transform = player_transform.translation.clone();
        target_transform.y = transform.translation.y;

        transform.look_at(target_transform, Vec3::Y);
    }
}
