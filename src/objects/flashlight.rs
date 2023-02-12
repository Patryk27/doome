use std::f32::consts::PI;

use crate::prelude::*;

#[derive(Component)]
pub struct Flashlight;

impl Flashlight {
    pub fn spawn(commands: &mut Commands) {
        commands.spawn((
            Transform::default(),
            Light {
                enabled: true,
                intensity: 1.0,
                kind: LightKind::Spot {
                    point_at: vec3(0.0, 0.0, 0.0),
                    angle: PI / 4.5,
                },
            },
            // Color::hex(0xffffff) * 0.8,
            Flashlight,
            Fade::fade_in(0.1),
        ));
    }

    pub fn sync_with_player(
        camera: Query<&Camera>,
        mut flashlight: Query<(&mut Transform, &mut Light, &Flashlight)>,
    ) {
        // let Ok(camera) = camera.get_single() else { return };
        // let Ok((mut fl_transform, mut fl_light, _)) = flashlight.get_single_mut() else { return };

        // fl_transform.translation = camera.origin + vec3(0.0, -0.1, 0.0);

        // *fl_light.point_at_mut().unwrap() = camera.look_at;
    }
}
