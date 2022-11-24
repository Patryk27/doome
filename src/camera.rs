use crate::prelude::*;

pub fn sync_with_player(
    mut camera: Query<&mut Camera>,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(mut camera) = camera.get_single_mut() else { return };
    let Ok(transform) = player.get_single() else { return };
    let pos = transform.translation;

    camera.origin = vec3(pos.x, 1.2, pos.z);
    camera.look_at = camera.origin + transform.forward() * 5.0;
}
