use std::f32::consts::PI;

use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::components::*;
use doome_bevy::physics::{Body, BodyType, CircleCollider, Collider};
use glam::vec3;

use super::utils::*;
use crate::player::Player;
use crate::ui_and_2d::Print;

const MAIN_CENTER_Z: f32 = (21.0 + 37.0) / 2.0;
const ELEPHANT_Z: f32 = MAIN_CENTER_Z + 3.0;

pub fn init(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        Player { can_move: false },
        Transform::from_rotation(Quat::from_rotation_x(PI)),
        Body {
            velocity: vec3(0.0, 0.0, 0.0),
            body_type: BodyType::Kinematic,
        },
        Collider::Circle(CircleCollider { radius: 0.5 }),
    ));

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    // Corridor
    lvl.floor(-1, -1, 1, 20).spawn();
    lvl.wall(1, -1, 1, 20, 1).spawn();
    lvl.wall(-1, -1, 1, -1, 2).spawn();
    lvl.wall(-1, -1, -1, 20, 3).spawn();

    let sl_corr0 = lvl
        .spot_light(
            vec3(0.0, 4.0, 0.0),
            vec3(0.0, 0.0, 0.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::in_delayed(0.5, 0.1))
        .id();

    let sl_corr1 = lvl
        .spot_light(
            vec3(0.0, 4.0, 6.0),
            vec3(0.0, 0.0, 6.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::in_delayed(1.5, 0.1))
        .id();

    let sl_corr2 = lvl
        .spot_light(
            vec3(0.0, 4.0, 12.0),
            vec3(0.0, 0.0, 12.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::in_delayed(2.5, 0.1))
        .id();

    let sl_corr3 = lvl
        .spot_light(
            vec3(0.0, 4.0, 18.0),
            vec3(0.0, 0.0, 18.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::in_delayed(3.5, 0.1))
        .id();

    // Main
    lvl.floor(-8, 21, 8, 37).spawn();
    lvl.wall(-8, 37, 8, 37, 0).spawn();
    lvl.wall(8, 37, 8, 21, 1).spawn();
    lvl.wall(-8, 21, -2, 21, 2).spawn();
    lvl.wall(8, 21, 2, 21, 2).spawn();
    lvl.wall(-8, 37, -8, 21, 3).spawn();

    lvl.model("elephant")
        .with_translation(vec3(0.0, 1.0, ELEPHANT_Z))
        .with_scale(Vec3::splat(0.4))
        .with_rotation(Quat::from_rotation_y(PI / 2.0))
        .with_material(
            Material::default()
                .with_color(Color::hex(0x789fbb))
                .with_reflectivity(0.8)
                .with_reflection_color(Color::hex(0xffffff)),
        )
        .spawn();

    let sl_main0 = lvl
        .spot_light(
            vec3(0.0, 4.0, ELEPHANT_Z - 4.0),
            vec3(0.0, 0.0, ELEPHANT_Z),
            PI / 2.0,
            Color::hex(0x990000),
            0.0,
        )
        .insert(LightFade::in_delayed(4.5, 0.1))
        .id();

    lvl.complete(Level {
        stage: LevelStage::CorridorIntro {
            tt: 0.0,
            sl_corr0,
            sl_corr1,
            sl_corr2,
            sl_corr3,
            sl_main0,
        },
    });
}

#[derive(Resource)]
pub struct Level {
    stage: LevelStage,
}

enum LevelStage {
    CorridorIntro {
        tt: f32,
        sl_corr0: Entity,
        sl_corr1: Entity,
        sl_corr2: Entity,
        sl_corr3: Entity,
        sl_main0: Entity,
    },

    Corridor {
        sl_corr0: Entity,
        sl_corr1: Entity,
        sl_corr2: Entity,
        sl_corr3: Entity,
        sl_main0: Entity,
    },

    MainIntro0 {
        tt: f32,
    },

    MainIntro1 {
        tt: f32,
    },

    Main,
}

pub fn process(
    assets: Res<Assets>,
    time: Res<Time>,
    mut commands: Commands,
    mut level: ResMut<Level>,
    mut player: Query<&mut Player>,
    camera: Query<&Camera>,
    mut print_tx: EventWriter<Print>,
) {
    match &mut level.stage {
        LevelStage::CorridorIntro {
            tt,
            sl_corr0,
            sl_corr1,
            sl_corr2,
            sl_corr3,
            sl_main0,
        } => {
            *tt += time.delta_seconds();

            if *tt < 4.5 {
                return;
            }

            player.single_mut().can_move = true;

            level.stage = LevelStage::Corridor {
                sl_corr0: *sl_corr0,
                sl_corr1: *sl_corr1,
                sl_corr2: *sl_corr2,
                sl_corr3: *sl_corr3,
                sl_main0: *sl_main0,
            };

            print_tx.send(Print::new(
                "Hello, Neo.\n\
                \n\
                Move using WASD + mouse or comma / period.\n\
                Shoot using space.\n\
                \n\
                The rest, find out you will.",
            ));
        }

        LevelStage::Corridor {
            sl_corr0,
            sl_corr1,
            sl_corr2,
            sl_corr3,
            sl_main0,
        } => {
            let camera = camera.single().origin;

            if (-3.0..=3.0).contains(&camera.x)
                && (19.0..=20.0).contains(&camera.z)
            {
                for sl in [sl_corr0, sl_corr1, sl_corr2, sl_corr3] {
                    commands.entity(*sl).insert(LightFade::out(1.0));
                }

                commands
                    .entity(*sl_main0)
                    .insert(LightFade::out_delayed(1.5, 0.5));

                let mut lvl = LevelBuilder::new(&mut commands, &assets);

                for x in [-8.4, 8.4] {
                    lvl.spot_light(
                        vec3(x, 2.5, ELEPHANT_Z - 8.4),
                        vec3(0.0, 0.0, ELEPHANT_Z),
                        PI / 3.0,
                        Color::hex(0xffffff),
                        0.0,
                    )
                    .insert(LightFade::in_delayed(3.0, 0.15));
                }

                player.single_mut().can_move = false;

                level.stage = LevelStage::MainIntro0 { tt: 0.0 };
            }
        }

        LevelStage::MainIntro0 { tt } => {
            *tt += time.delta_seconds();

            if *tt < 2.0 {
                return;
            }

            LevelBuilder::new(&mut commands, &assets)
                .ceiling(-8, 18, 8, 37)
                .dynamic()
                .spawn();

            level.stage = LevelStage::MainIntro1 { tt: 0.0 };
        }

        LevelStage::MainIntro1 { tt } => {
            *tt += time.delta_seconds();

            if *tt < 1.5 {
                return;
            }

            player.single_mut().can_move = true;

            level.stage = LevelStage::Main;
        }

        LevelStage::Main => {
            //
        }
    }
}
