use crate::prelude::*;

#[derive(Component)]
pub struct Door;

impl Door {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec3,
        rotation: Quat,
    ) -> Entity {
        let model = assets.load_model("door");
        let position = position + vec3(0.0, 1.0, 0.0);

        commands
            .spawn((
                model,
                Transform::from_translation(position).with_rotation(rotation),
                GeometryType::Dynamic,
                Material::default()
                    .two_sided()
                    .with_color(Color::hex(0xffffff))
                    .with_uv_transparency(),
                Collider::line(vec2(0.0, -1.0), vec2(0.0, 1.0)),
            ))
            .id()
    }
}
