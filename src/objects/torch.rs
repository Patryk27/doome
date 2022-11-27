use std::f32::consts::PI;
use std::time::Duration;

use crate::prelude::*;

#[derive(Component)]
pub struct Torch {
    active: bool,
    force_active_texture: bool,
    position: Vec2,
    rotation: Quat,
    intensity: Animation,
}

impl Torch {
    pub fn new() -> Self {
        Self {
            active: true,
            force_active_texture: false,
            position: Default::default(),
            rotation: Default::default(),
            intensity: Animation::GoingUp {
                to: 1.0,
                speed: 0.5,
            },
        }
    }

    pub fn with_active(mut self, val: bool) -> Self {
        self.active = val;

        self.intensity = Animation::GoingDown {
            to: 0.0,
            speed: 1.0,
        };

        self
    }

    pub fn with_force_active_texture(mut self, val: bool) -> Self {
        self.force_active_texture = val;
        self
    }

    pub fn with_position(mut self, val: Vec2) -> Self {
        self.position = val;
        self
    }

    pub fn with_rotation(mut self, val: Quat) -> Self {
        self.rotation = val;
        self
    }

    pub fn spawn(self, assets: &Assets, commands: &mut Commands) -> Entity {
        let active = self.active;
        let force_active_texture = self.force_active_texture;
        let position = vec3(self.position.x, 2.3, self.position.y);
        let rotation = self.rotation;

        let mut torch = commands.spawn((
            self,
            Light {
                enabled: true,
                intensity: 0.0,
                kind: LightKind::Point,
            },
            Transform::from_translation(position),
            Color::hex(0xff7714),
        ));

        if active {
            torch.insert(TorchActive::now());
        }

        torch.add_children(|commands| {
            let model = assets.load_model("wall");

            let texture =
                assets.load_texture(if active || force_active_texture {
                    "torch"
                } else {
                    "torch.off"
                });

            commands.spawn((
                TorchTexture,
                model,
                Transform::from_translation(
                    position - rotation * vec3(0.3, 0.0, 0.0),
                )
                .with_rotation(rotation * Quat::from_rotation_z(PI))
                .with_scale(Vec3::splat(0.35)),
                GeometryType::Dynamic,
                Material::default()
                    .double_sided()
                    .emissive()
                    .with_color(Color::hex(0xffffff))
                    .with_texture(texture)
                    .with_uv_transparency()
                    .without_casting_shadows(),
            ));
        });

        torch.id()
    }

    pub(super) fn process(
        time: Res<Time>,
        assets: Res<Assets>,
        mut torches: Query<(
            &mut Torch,
            &mut Light,
            Option<&mut TorchActive>,
            &Children,
        )>,
        mut torch_textures: Query<&mut Material, With<TorchTexture>>,
    ) {
        let dt = time.delta();
        let dts = time.delta_seconds();

        let rand = ((time.elapsed_seconds_f64() * 100000.0) as i32)
            .reflect_hash()
            .unwrap();

        let frand = (rand % 100) as f32 / 100.0;

        for (mut torch, mut torch_light, torch_active, children) in
            torches.iter_mut()
        {
            let should_be_active = if let Some(mut torch_active) = torch_active
            {
                if torch_active.delay.finished() {
                    true
                } else {
                    torch_active.delay.tick(dt);
                    false
                }
            } else {
                false
            };

            if torch.active != should_be_active {
                torch.active = should_be_active;

                if torch.active {
                    torch.intensity = Animation::GoingUp {
                        to: 1.0,
                        speed: 0.85,
                    };
                } else {
                    torch.intensity = Animation::GoingDown {
                        to: 0.0,
                        speed: 0.75,
                    };
                }

                if !torch.force_active_texture {
                    let torch_texture = children.iter().next().unwrap();

                    let mut torch_texture =
                        torch_textures.get_mut(*torch_texture).unwrap();

                    torch_texture.texture =
                        Some(assets.load_texture(if torch.active {
                            "torch"
                        } else {
                            "torch.off"
                        }));
                }
            }

            match torch.intensity {
                Animation::GoingUp { to, speed } => {
                    torch_light.intensity += dts * speed * 1.25;

                    if torch_light.intensity > to {
                        torch.intensity = Animation::GoingDown {
                            to: (torch_light.intensity - frand).max(0.35),
                            speed: 0.8 + frand,
                        };
                    }
                }

                Animation::GoingDown { to, speed } => {
                    torch_light.intensity -= dts * speed * 1.25;

                    if torch_light.intensity < to {
                        if torch.active {
                            torch.intensity = Animation::GoingUp {
                                to: (torch_light.intensity + frand).min(1.2),
                                speed: 0.8 + frand,
                            };
                        } else if torch_light.intensity < 0.0 {
                            torch_light.intensity = 0.0;
                            torch.intensity = Animation::Stopped;
                        }
                    }
                }

                Animation::Stopped => {
                    //
                }
            }
        }
    }
}

#[derive(Component)]
pub struct TorchActive {
    pub delay: Timer,
}

impl TorchActive {
    pub fn now() -> Self {
        Self::in_ms(0)
    }

    pub fn in_ms(ms: u64) -> Self {
        Self {
            delay: Timer::new(Duration::from_millis(ms), TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub(super) struct TorchTexture;

enum Animation {
    GoingUp { to: f32, speed: f32 },
    GoingDown { to: f32, speed: f32 },
    Stopped,
}
