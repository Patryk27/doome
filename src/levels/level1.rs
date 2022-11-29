use std::f32::consts::PI;

use indoc::indoc;

use super::builder::*;
use crate::prelude::*;

const INTRO_TEXT: &str = indoc! {r#"
    Hello, Neo.

    The world needs your help... again!%{s0.50}

    Move using WASD + mouse or comma / period.
    Shoot using space.

    The rest, find out you will.
"#};

const OUTRO_TEXT: &str = indoc! {r#"
    So:

    On planet Earth, in Area 51, there is a desert.
    %{s0.25}
    On that desert, there stands a fridge with an
    LCD panel.
    %{s0.25}
    On that panel, there is Windows 4.20.
    %{s0.25}
    On its desktop? %{s1.0}






    DOOM






    %{s2.0}







    %{c}Mr Freeman, our intel says someone wants to
    destroy that fridge, obliterating DOOM from
    existence -- will you help us?%{s1.0}

    Great!

    In a moment we will connect you to a hacking
    terminal - your mission?%{s1.0}

    Save the world.%{s0.8}

    !! DON'T DIE !!
    (( break a leg ))

    $ ping google.com%{h}
    PING google.com (274.271.61.64) 64(64) bytes
    64 bytes from 1337.g00gl3.net: time=1 ms
    64 bytes from 1337.g00gl3

    ERROR: communication taken over by a
    virus...
"#};

const ROOM_CENTER_Z: f32 = (21.0 + 37.0) / 2.0;
const ELEPHANT_Z: f32 = ROOM_CENTER_Z + 3.0;

pub fn init(
    mut commands: Commands,
    assets: Res<Assets>,
    mut goto_level_rx: EventReader<GotoLevel>,
    mut player: Query<(&mut Player, &mut Transform)>,
) {
    if !goto_level_rx.iter().any(|level| **level == Level::l1()) {
        return;
    }

    // -----

    let (mut player, mut player_xform) = player.single_mut();

    player.can_move = false;

    *player_xform =
        Transform::default().with_rotation(Quat::from_rotation_y(PI));

    // -----

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    lvl.floor(-1, -1, 1, 20)
        .alter_material(|mat| {
            mat.with_reflectivity(0.1)
                .with_reflection_color(Color::hex(0xffffff))
        })
        .spawn();

    lvl.wall(2, -1, 2, 20, 1).spawn();
    lvl.wall(-2, -1, 2, -1, 2).spawn();
    lvl.wall(-2, -1, -2, 20, 3).spawn();

    let corr_sl0 = lvl
        .spot_light(
            vec3(0.0, 4.0, 0.0),
            vec3(0.0, 0.0, 0.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::fade_in_delayed(0.5, 0.1))
        .id();

    let corr_sl1 = lvl
        .spot_light(
            vec3(0.0, 4.0, 6.0),
            vec3(0.0, 0.0, 6.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::fade_in_delayed(1.5, 0.1))
        .id();

    let corr_sl2 = lvl
        .spot_light(
            vec3(0.0, 4.0, 12.0),
            vec3(0.0, 0.0, 12.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::fade_in_delayed(2.5, 0.1))
        .id();

    let corr_sl3 = lvl
        .spot_light(
            vec3(0.0, 4.0, 18.0),
            vec3(0.0, 0.0, 18.0),
            PI / 4.0,
            Color::hex(0xffffff),
            0.0,
        )
        .insert(LightFade::fade_in_delayed(3.5, 0.1))
        .id();

    // -----

    lvl.floor(-8, 21, 8, 37)
        .alter_material(|mat| {
            mat.with_reflectivity(0.1)
                .with_reflection_color(Color::hex(0xffffff))
        })
        .spawn();

    lvl.ceiling(-8, 18, 8, 37)
        .alter_material(|mat| {
            mat.with_reflectivity(0.25)
                .with_reflection_color(Color::hex(0xffffff))
        })
        .spawn();

    lvl.wall(-9, 38, 9, 38, 0).spawn();
    lvl.wall(9, 38, 9, 21, 1).spawn();
    lvl.wall(-9, 20, -2, 20, 2).spawn();
    lvl.wall(9, 20, 2, 20, 2).spawn();
    lvl.wall(-9, 38, -9, 21, 3).spawn();

    lvl.model("elephant")
        .with_translation(vec3(0.0, 1.0, ELEPHANT_Z))
        .with_scale(Vec3::splat(0.32))
        .with_rotation(Quat::from_rotation_y(PI / 2.0))
        .with_material(
            Material::default()
                .with_color(Color::hex(0x789fbb))
                .with_reflectivity(0.8)
                .with_reflection_color(Color::hex(0xffffff)),
        )
        .with_collider(Collider::circle(4.0, 16))
        .spawn();

    let room_sl0 = lvl
        .spot_light(
            vec3(0.0, 1.0, ELEPHANT_Z - 4.0),
            vec3(0.0, 0.0, ELEPHANT_Z),
            PI / 2.0,
            Color::hex(0xff0000),
            0.0,
        )
        .insert(LightFade::fade_in_delayed(4.5, 0.1))
        .id();

    lvl.complete(LevelState {
        stage: LevelStage::Intro {
            tt: 0.0,
            corr_sl0,
            corr_sl1,
            corr_sl2,
            corr_sl3,
            room_sl0,
        },
    });
}

#[derive(Component)]
pub struct LevelState {
    stage: LevelStage,
}

enum LevelStage {
    Intro {
        tt: f32,
        corr_sl0: Entity,
        corr_sl1: Entity,
        corr_sl2: Entity,
        corr_sl3: Entity,
        room_sl0: Entity,
    },

    AwaitingRoomEnter {
        intro_completed: bool,
        corr_sl0: Entity,
        corr_sl1: Entity,
        corr_sl2: Entity,
        corr_sl3: Entity,
        room_sl0: Entity,
    },

    AwaitingIntroCompletion {
        txt_elephant: Entity,
        room_sl0: Entity,
        room_sl1: Entity,
    },

    InsideRoom {
        txt_elephant: Entity,
        room_sl0: Entity,
        room_sl1: Entity,
    },

    Outro,
}

pub fn process(
    assets: Res<Assets>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut level: Query<&mut LevelState>,
    mut player: Query<&mut Player>,
    camera: Query<&Camera>,
    mut typewriter_tx: EventWriter<TypewriterPrint>,
    mut typewriter_rx: EventReader<TypewriterPrintingCompleted>,
    mut visibilities: Query<&mut Visibility>,
    mut goto_level_tx: EventWriter<GotoLevel>,
    mut change_hud_visibility_tx: EventWriter<ChangeHudVisibility>,
) {
    let Ok(mut level) = level.get_single_mut() else { return };

    match &mut level.stage {
        LevelStage::Intro {
            tt,
            corr_sl0,
            corr_sl1,
            corr_sl2,
            corr_sl3,
            room_sl0,
        } => {
            if *tt == 0.0 {
                change_hud_visibility_tx.send(ChangeHudVisibility::hide());
            }

            *tt += time.delta_seconds();

            if *tt < 4.5 {
                return;
            }

            player.single_mut().can_move = true;

            typewriter_tx
                .send(TypewriterPrint::new(INTRO_TEXT).with_id("intro"));

            level.stage = LevelStage::AwaitingRoomEnter {
                intro_completed: false,
                corr_sl0: *corr_sl0,
                corr_sl1: *corr_sl1,
                corr_sl2: *corr_sl2,
                corr_sl3: *corr_sl3,
                room_sl0: *room_sl0,
            };
        }

        LevelStage::AwaitingRoomEnter {
            intro_completed,
            corr_sl0,
            corr_sl1,
            corr_sl2,
            corr_sl3,
            room_sl0,
        } => {
            if !*intro_completed {
                *intro_completed =
                    typewriter_rx.iter().any(|event| event.id == "intro");
            }

            let camera = camera.single().origin;

            if !(-3.0..=3.0).contains(&camera.x)
                || !(20.0..=21.0).contains(&camera.z)
            {
                return;
            }

            for sl in [corr_sl0, corr_sl1, corr_sl2, corr_sl3, room_sl0] {
                commands.entity(*sl).insert(LightFade::fade_out(0.25));
            }

            let mut lvl = LevelBuilder::new(&mut commands, &assets);

            let txt_elephant = lvl
                .commands()
                .spawn((
                    Text::new("Press F to address the elephant in the room")
                        .centered(),
                    Transform::from_translation(vec3(0.5, 0.05, 0.0)),
                    Visibility::invisible(),
                ))
                .id();

            let room_sl0 = lvl
                .spot_light(
                    vec3(-8.4, 2.0, ELEPHANT_Z - 8.4),
                    vec3(0.0, 0.0, ELEPHANT_Z),
                    PI / 3.0,
                    Color::hex(0xffffff),
                    0.0,
                )
                .insert(LightFade::fade_in_delayed(0.4, 0.15))
                .id();

            let room_sl1 = lvl
                .spot_light(
                    vec3(8.4, 2.0, ELEPHANT_Z - 8.4),
                    vec3(0.0, 0.0, ELEPHANT_Z),
                    PI / 3.0,
                    Color::hex(0xffffff),
                    0.0,
                )
                .insert(LightFade::fade_in_delayed(0.4, 0.15))
                .id();

            level.stage = if *intro_completed {
                LevelStage::InsideRoom {
                    txt_elephant,
                    room_sl0,
                    room_sl1,
                }
            } else {
                LevelStage::AwaitingIntroCompletion {
                    txt_elephant,
                    room_sl0,
                    room_sl1,
                }
            };
        }

        LevelStage::AwaitingIntroCompletion {
            txt_elephant,
            room_sl0,
            room_sl1,
        } => {
            let ready = typewriter_rx.iter().any(|event| event.id == "intro");

            if ready {
                level.stage = LevelStage::InsideRoom {
                    txt_elephant: *txt_elephant,
                    room_sl0: *room_sl0,
                    room_sl1: *room_sl1,
                };
            }
        }

        LevelStage::InsideRoom {
            txt_elephant,
            room_sl0,
            room_sl1,
        } => {
            let is_close_to_elephant =
                camera.single().origin.distance(vec3(0.0, 0.0, ELEPHANT_Z))
                    < 3.2;

            let mut txt_elephant_visibility =
                visibilities.get_mut(*txt_elephant).unwrap();

            txt_elephant_visibility.is_visible = is_close_to_elephant;

            if is_close_to_elephant && keys.just_pressed(KeyCode::F) {
                txt_elephant_visibility.is_visible = false;

                commands.entity(*room_sl0).insert(LightFade::fade_out(3.0));
                commands.entity(*room_sl1).insert(LightFade::fade_out(3.0));

                level.stage = LevelStage::Outro;

                player.single_mut().can_move = false;

                typewriter_tx
                    .send(TypewriterPrint::new(OUTRO_TEXT).with_id("outro"));
            }
        }

        LevelStage::Outro => {
            let ready = typewriter_rx.iter().any(|event| event.id == "outro");

            if ready {
                goto_level_tx.send(GotoLevel::new(Level::l2()));
            }
        }
    }
}
