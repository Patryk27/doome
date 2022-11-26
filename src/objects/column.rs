use crate::prelude::*;

#[derive(Component)]
pub struct Column;

impl Column {
    pub fn spawn(
        assets: &Assets,
        commands: &mut Commands,
        position: Vec2,
    ) -> Entity {
        let model = assets.load_model("column");
        let position = vec3(position.x, 0.0, position.y);

        commands
            .spawn((
                model,
                Transform::from_translation(position)
                    .with_scale(vec3(0.25, 0.33, 0.25)),
                GeometryType::Static,
                Material::default().with_color(Color::hex(0xffffff)),
                Collider::line(vec2(0.0, -1.0), vec2(0.0, 1.0)),
            ))
            .id()
    }
}
