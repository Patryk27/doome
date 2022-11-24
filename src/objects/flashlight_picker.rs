use std::f32::consts::PI;

use crate::prelude::*;

#[derive(Component)]
pub struct FlashlightPicker;

impl FlashlightPicker {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec3,
    ) -> Entity {
        let model = assets.load_model("flashlight");

        commands
            .spawn((
                model,
                Transform {
                    translation: position + vec3(0.0, 0.2, 0.0),
                    rotation: Quat::from_rotation_x(-PI / 2.0)
                        * Quat::from_rotation_z(-PI / 3.0),
                    scale: vec3(1.0, 1.0, 1.0),
                },
                Material::default().with_color(Color::hex(0xffffff)),
                Collider::circle(3.2),
                GeometryType::Static,
            ))
            .id()
    }
}
