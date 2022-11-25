use crate::commands::{Command, EntityOrPlayer};
use crate::pickable::Pickable;
use crate::prelude::*;

#[derive(Component)]
pub struct Heart;

impl Heart {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec3,
    ) -> Entity {
        let model = assets.load_model("heart");
        let position = position + vec3(0.0, 1.0, 0.0);

        commands
            .spawn((
                model,
                Transform::from_translation(position)
                    .with_scale(Vec3::splat(0.75)),
                Float {
                    anchor: position,
                    amplitude: 0.5,
                    speed: 1.0,
                },
                Billboard,
                GeometryType::Dynamic,
                Material::default()
                    .with_color(Color::hex(0xffffff))
                    .with_uv_transparency(),
                Collider::circle(1.25, 6),
                Pickable {
                    on_pickup: Command::Heal {
                        entity: EntityOrPlayer::Player,
                        amount: 20.0,
                    },
                },
            ))
            .id()
    }
}
