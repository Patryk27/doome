use bevy::prelude::*;

#[derive(Component)]
pub struct Rotate {
    pub inverted: bool,
    pub speed: f32, // in radians per second
}

#[derive(Component)]
pub struct Float {
    pub anchor: Vec3,
    pub amplitude: f32,
    pub speed: f32,
}

pub fn rotate(time: Res<Time>, mut items: Query<(&Rotate, &mut Transform)>) {
    for (rotate, mut transform) in items.iter_mut() {
        let delta = rotate.speed * time.delta_seconds();
        let delta = if rotate.inverted { -delta } else { delta };
        transform.rotate(Quat::from_rotation_y(delta));
    }
}

pub fn float(time: Res<Time>, mut items: Query<(&Float, &mut Transform)>) {
    for (float, mut transform) in items.iter_mut() {
        let y = float.anchor.y
            + float.amplitude * (time.elapsed_seconds() * float.speed).sin();

        transform.translation.y = y;
    }
}
