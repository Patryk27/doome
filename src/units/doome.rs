use std::sync::Arc;

use palette::{Gradient, LinSrgb};

use crate::prelude::*;
use crate::rng::RngState;
use crate::weapons::Weapon;

#[derive(Component)]
pub struct Doome {
    gradient: Gradient<LinSrgb>,
    gradient_progress: f32,
}

impl Doome {
    pub fn spawn(
        assets: &DoomeAssets,
        commands: &mut Commands,
        position: Vec3,
    ) -> Entity {
        let this = Self {
            gradient: Gradient::from(vec![
                (0.0, LinSrgb::new(0.00, 0.05, 0.20)),
                (1.0, LinSrgb::new(0.95, 0.90, 0.30)),
            ]),
            gradient_progress: 0.0,
        };

        let model = assets.load_model("moth_monster");
        let weapon = crate::weapons::definitions::doome_fire_spew(&assets);
        let texture = assets.load_texture("enemy.doome");

        commands
            .spawn((
                this,
                Enemy::new().with_follows_player(false),
                model,
                Material::default()
                    .with_texture(texture)
                    .with_uv_transparency(),
                GeometryType::Dynamic,
                Transform::from_translation(position)
                    .with_scale(Vec3::splat(0.0)),
                Billboard,
                Health::new(2500.0, 2500.0),
                RayCast {
                    origin: Vec2::ZERO,
                    direction: Vec2::NEG_Y * 20.0,
                    hit: None,
                },
                Collider::circle(1.0, 12),
                Body {
                    acceleration: Vec2::ZERO,
                    velocity: Vec2::ZERO,
                    body_type: BodyType::Kinematic,
                },
                Weapon::new(Arc::new(weapon)),
            ))
            .id()
    }

    pub(super) fn animate(
        mut rng: ResMut<RngState>,
        time: Res<Time>,
        mut doome: Query<(&mut Self, &mut Transform, &mut Material)>,
    ) {
        let Ok((mut this, mut xform, mut mat)) = doome.get_single_mut() else { return };

        if xform.scale.x < 8.0 {
            xform.scale =
                Vec3::splat(xform.scale.x + time.delta_seconds() * 2.5);
        }

        xform.translation.y = time.delta_seconds().sin() * 2.5;

        this.gradient_progress += time.delta_seconds() * 5.0;

        if this.gradient_progress > 1.0 {
            this.gradient = Gradient::from(vec![
                (0.0, this.gradient.get(1.0)),
                (1.0, LinSrgb::new(rng.gen(), rng.gen(), rng.gen())),
            ]);

            this.gradient_progress = 0.0;
        }

        let color = this.gradient.get(this.gradient_progress);

        mat.color = Some(Color {
            r: color.red,
            g: color.green,
            b: color.blue,
        });
    }
}
