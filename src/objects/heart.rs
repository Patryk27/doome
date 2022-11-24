use crate::prelude::*;

#[derive(Component)]
pub struct Heart;

impl Heart {
    pub fn spawn(
        commands: &mut Commands,
        assets: &Assets,
        position: Vec3,
    ) -> Entity {
        let model = assets.load_model("heart");

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
                    amplitude: 0.5,
                    speed: 1.0,
                },
                GeometryType::Dynamic,
                Material::default()
                    .with_reflectivity(1.0)
                    .with_color(Color::hex(0x000000))
                    .with_reflection_color(Color::hex(0xf12421)),
            ))
            .id()
    }
}
