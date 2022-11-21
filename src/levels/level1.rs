use std::f32::consts::PI;

use bevy::prelude::*;
use doome_bevy::assets::Assets;
use doome_bevy::billboard::Billboard;
use doome_bevy::components::*;
use doome_bevy::enemies::{Enemy, RecalculateNavData};
use doome_bevy::physics::components::{Body, BodyType, Collider};
use doome_bevy::player::Player;
use glam::vec3;
use indoc::indoc;

use super::utils::*;
use crate::ui::{Text, TypewriterPrint};

const INTRO_TEXT: &str = indoc! {r#"
    Hello, Neo.

    Move using WASD + mouse or comma / period.
    Shoot using space.

    The rest, find you you will.
"#};

const OUTRO_TEXT: &str = indoc! {r#"
    long time ago, in a galaxy not so far away
    (( planet earth, actually, area 51 ))

    there was a fridge
    %{s0.25}
    that fridge had an LCD panel, running ubuntu
    0.69
    %{s0.25}
    on that ubuntu, a minecraft server, running
    a redstone computer
    %{s0.25}
    on that redstone computer, a virtual machine
    emulating windows 4.20
    %{s0.25}
    in there? %{s2.0}






    DOOM






    %{s2.0}







    %{c}mr freeman, we are now connecting you to
    the terminal that will allow you to play that
    game

    your mission?%{s1.0}

    save the world.%{s0.8}

    break a leg

    $ ping google.com%{h}
    PING google.com (274.271.61.64) 64(64) bytes
    64 bytes from 1337.g00gl3.net: time=1 ms
    64 bytes from 1337.g00gl3.net: time=2 ms
    64 bytes from 1337.g00gl3.net: time=3 ms
"#};

const MAIN_CENTER_Z: f32 = (21.0 + 37.0) / 2.0;
const ELEPHANT_Z: f32 = MAIN_CENTER_Z + 3.0;

pub fn init(
    mut commands: Commands,
    assets: Res<Assets>,
    mut recalc_nav_data: EventWriter<RecalculateNavData>,
) {
    commands.spawn((
        Player { can_move: false },
        Transform::from_rotation(Quat::from_rotation_x(PI)),
        Body {
            velocity: Vec2::ZERO,
            body_type: BodyType::Kinematic,
        },
        Collider::circle(0.5),
    ));

    let moth_model = assets.load_model("moth_monster");

    commands.spawn((
        Enemy::default(),
        moth_model,
        Material::default().with_uv_transparency(),
        GeometryType::Dynamic,
        Transform::from_translation(vec3(0.0, 0.0, ELEPHANT_Z + 1.0)),
        Billboard,
        // Collider::circle(0.5),
    ));

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    // Corridor
    lvl.floor(-1, -1, 1, 20).spawn();
    lvl.wall(1, -1, 1, 20, 1).spawn();
    lvl.wall(-1, -1, 1, -1, 2).spawn();
    lvl.wall(-1, -1, -1, 20, 3).spawn();

    let corr_sl0 = lvl
        .spot_light(
            vec3(0.0, 4.0, 0.0),
            vec3(0.0, 0.0, 0.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::in_delayed(0.5, 0.1))
        .id();

    let corr_sl1 = lvl
        .spot_light(
            vec3(0.0, 4.0, 6.0),
            vec3(0.0, 0.0, 6.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::in_delayed(1.5, 0.1))
        .id();

    let corr_sl2 = lvl
        .spot_light(
            vec3(0.0, 4.0, 12.0),
            vec3(0.0, 0.0, 12.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::in_delayed(2.5, 0.1))
        .id();

    let corr_sl3 = lvl
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
    lvl.floor(-8, 21, 8, 37).dynamic().spawn();
    lvl.wall(-8, 37, 8, 37, 0).dynamic().spawn();
    lvl.wall(8, 37, 8, 21, 1).dynamic().spawn();
    lvl.wall(-8, 21, -2, 21, 2).dynamic().spawn();
    lvl.wall(8, 21, 2, 21, 2).dynamic().spawn();
    lvl.wall(-8, 37, -8, 21, 3).dynamic().spawn();

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

    let main_sl0 = lvl
        .spot_light(
            vec3(0.0, 4.0, ELEPHANT_Z - 4.0),
            vec3(0.0, 0.0, ELEPHANT_Z),
            PI / 2.0,
            Color::hex(0x990000),
            0.0,
        )
        .insert(LightFade::in_delayed(4.5, 0.1))
        .id();

    recalc_nav_data.send(RecalculateNavData);

    lvl.complete(Level {
        stage: LevelStage::CorridorIntro {
            tt: 0.0,
            corr_sl0,
            corr_sl1,
            corr_sl2,
            corr_sl3,
            main_sl0,
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
        corr_sl0: Entity,
        corr_sl1: Entity,
        corr_sl2: Entity,
        corr_sl3: Entity,
        main_sl0: Entity,
    },

    Corridor {
        corr_sl0: Entity,
        corr_sl1: Entity,
        corr_sl2: Entity,
        corr_sl3: Entity,
        main_sl0: Entity,
    },

    MainIntro0 {
        tt: f32,
        main_sl0: Entity,
        main_sl1: Entity,
    },

    MainIntro1 {
        tt: f32,
        main_sl0: Entity,
        main_sl1: Entity,
    },

    Main {
        txt_elephant: Entity,
        main_sl0: Entity,
        main_sl1: Entity,
    },

    Outro {
        tt: f32,
    },
}

pub fn process(
    assets: Res<Assets>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut level: ResMut<Level>,
    mut player: Query<&mut Player>,
    camera: Query<&Camera>,
    mut print_tx: EventWriter<TypewriterPrint>,
    mut visibilities: Query<&mut Visibility>,
) {
    match &mut level.stage {
        LevelStage::CorridorIntro {
            tt,
            corr_sl0,
            corr_sl1,
            corr_sl2,
            corr_sl3,
            main_sl0,
        } => {
            *tt += time.delta_seconds();

            if *tt < 4.5 {
                return;
            }

            player.single_mut().can_move = true;

            level.stage = LevelStage::Corridor {
                corr_sl0: *corr_sl0,
                corr_sl1: *corr_sl1,
                corr_sl2: *corr_sl2,
                corr_sl3: *corr_sl3,
                main_sl0: *main_sl0,
            };

            print_tx.send(TypewriterPrint::new(INTRO_TEXT));
        }

        LevelStage::Corridor {
            corr_sl0,
            corr_sl1,
            corr_sl2,
            corr_sl3,
            main_sl0,
        } => {
            let camera = camera.single().origin;

            if (-3.0..=3.0).contains(&camera.x)
                && (19.0..=20.0).contains(&camera.z)
            {
                for sl in [corr_sl0, corr_sl1, corr_sl2, corr_sl3] {
                    commands.entity(*sl).insert(LightFade::out(1.0));
                }

                commands
                    .entity(*main_sl0)
                    .insert(LightFade::out_delayed(1.5, 0.5));

                let mut lvl = LevelBuilder::new(&mut commands, &assets);

                let main_sl0 = lvl
                    .spot_light(
                        vec3(-8.4, 2.5, ELEPHANT_Z - 8.4),
                        vec3(0.0, 0.0, ELEPHANT_Z),
                        PI / 3.0,
                        Color::hex(0xffffff),
                        0.0,
                    )
                    .insert(LightFade::in_delayed(3.0, 0.15))
                    .id();

                let main_sl1 = lvl
                    .spot_light(
                        vec3(8.4, 2.5, ELEPHANT_Z - 8.4),
                        vec3(0.0, 0.0, ELEPHANT_Z),
                        PI / 3.0,
                        Color::hex(0xffffff),
                        0.0,
                    )
                    .insert(LightFade::in_delayed(3.0, 0.15))
                    .id();

                player.single_mut().can_move = false;

                level.stage = LevelStage::MainIntro0 {
                    tt: 0.0,
                    main_sl0,
                    main_sl1,
                };
            }
        }

        LevelStage::MainIntro0 {
            tt,
            main_sl0,
            main_sl1,
        } => {
            *tt += time.delta_seconds();

            if *tt < 2.0 {
                return;
            }

            LevelBuilder::new(&mut commands, &assets)
                .ceiling(-8, 18, 8, 37)
                .dynamic()
                .spawn();

            level.stage = LevelStage::MainIntro1 {
                tt: 0.0,
                main_sl0: *main_sl0,
                main_sl1: *main_sl1,
            };
        }

        LevelStage::MainIntro1 {
            tt,
            main_sl0,
            main_sl1,
        } => {
            *tt += time.delta_seconds();

            if *tt < 1.5 {
                return;
            }

            player.single_mut().can_move = true;

            let txt_elephant = commands
                .spawn((
                    Text::new("Press F to address the elephant in the room")
                        .centered(),
                    Transform::from_translation(vec3(0.5, 0.05, 0.0)),
                    Visibility::invisible(),
                ))
                .id();

            level.stage = LevelStage::Main {
                txt_elephant,
                main_sl0: *main_sl0,
                main_sl1: *main_sl1,
            };
        }

        LevelStage::Main {
            txt_elephant,
            main_sl0,
            main_sl1,
        } => {
            let is_close_to_elephant =
                camera.single().origin.distance(vec3(0.0, 0.0, ELEPHANT_Z))
                    < 3.2;

            let mut txt_elephant_visibility =
                visibilities.get_mut(*txt_elephant).unwrap();

            txt_elephant_visibility.is_visible = is_close_to_elephant;

            commands.entity(*main_sl0).insert(LightFade::out(3.0));
            commands.entity(*main_sl1).insert(LightFade::out(3.0));

            if is_close_to_elephant && keys.just_pressed(KeyCode::F) {
                txt_elephant_visibility.is_visible = false;
                level.stage = LevelStage::Outro { tt: 0.0 };
                player.single_mut().can_move = true;
                print_tx.send(TypewriterPrint::new(OUTRO_TEXT));
            }
        }

        LevelStage::Outro { tt } => {
            //
        }
    }
}
