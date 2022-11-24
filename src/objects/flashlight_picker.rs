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
        let position = position + vec3(0.0, 0.9, 0.0);

        commands
            .spawn((
                model,
                Transform::from_translation(position),
                Rotate {
                    inverted: false,
                    speed: 0.4,
                },
                Float {
                    anchor: position,
                    amplitude: 0.4,
                    speed: 2.0,
                },
                Material::default()
                    .with_color(Color::hex(0xffffff))
                    .with_uv_transparency(),
                GeometryType::Dynamic,
                Collider::circle(1.25, 6),
            ))
            .id()
    }
}
