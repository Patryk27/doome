use crate::prelude::*;

#[derive(Component)]
pub struct Gate;

impl Gate {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec3,
        rotation: Quat,
    ) -> Entity {
        let model = assets.load_model("gate");

        commands
            .spawn((
                model,
                Transform::from_translation(position).with_rotation(rotation),
                GeometryType::Static,
                Material::default()
                    .with_color(Color::hex(0xffffff) * 0.75)
                    .with_uv_transparency(),
                Collider::line(vec2(0.0, -1.0), vec2(0.0, 1.0)),
            ))
            .id()
    }
}
