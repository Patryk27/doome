use crate::prelude::*;

#[derive(Component)]
pub struct Gate;

impl Gate {
    pub fn spawn(
        assets: &DoomeAssets,
        commands: &mut Commands,
        position: Vec2,
        rotation: Quat,
    ) -> Entity {
        let model = assets.load_model("gate");
        let position = vec3(position.x, 0.0, position.y);

        commands
            .spawn((
                model,
                Transform::from_translation(position).with_rotation(rotation),
                GeometryType::Static,
                Material::default()
                    .double_sided()
                    .with_color(Color::hex(0xffffff) * 0.75)
                    .with_uv_transparency(),
                Collider::line(vec2(0.0, -1.0), vec2(0.0, 1.0)),
            ))
            .id()
    }
}
