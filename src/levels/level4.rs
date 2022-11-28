use std::f32::consts::PI;
use std::time::Duration;

use crate::music::MusicTrack;
use crate::prelude::*;

pub fn init(
    mut commands: Commands,
    mut game_commands: EventWriter<Command>,
    assets: Res<Assets>,
    mut goto_level_rx: EventReader<GotoLevel>,
    mut player: Query<(&mut Player, &mut Transform)>,
) {
    if !goto_level_rx.iter().any(|level| **level == Level::l4()) {
        return;
    }

    // -----

    let (mut player, mut player_xform) = player.single_mut();

    player.can_move = true;
    *player_xform = Transform::default().with_translation(vec3(0.0, 0.0, 2.0));

    // -----

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    let l0 = lvl
        .point_light(Default::default(), Color::hex(0x0a0c37) * 1.0, 0.0)
        .id();

    let l1 = lvl
        .point_light(Default::default(), Color::hex(0xff9900) * 0.4, 0.0)
        .id();

    let l2 = lvl
        .point_light(Default::default(), Color::hex(0xff61c6) * 0.3, 0.0)
        .id();

    let locator =
        LevelLoader::load(include_str!("../../assets/levels/level4.tmj"))
            .spawn(&mut lvl);

    for column in 1..=4 {
        let column_pos = locator.tag(format!("column-{}", column));

        Column::spawn(lvl.assets(), lvl.commands(), column_pos.xz());
    }

    game_commands.send(Command::SwitchTrack {
        track: MusicTrack::Chillout,
    });

    lvl.model("wall")
        .with_translation(vec3(0.0, 2.0, -2.4))
        .with_rotation(Quat::from_rotation_x(PI))
        .with_scale(Vec3::splat(0.8))
        .with_material(
            Material::default()
                .double_sided()
                .with_color(Color::hex(0xffffff))
                .with_texture(assets.load_texture("wall.david"))
                .without_casting_shadows(),
        )
        .spawn();

    // -----

    lvl.complete(LevelState {
        l0,
        l1,
        l2,
        stage: LevelStage::Booting {
            tt: 0.0,
            timer: Timer::new(Duration::from_secs(6), TimerMode::Once),
        },
    });
}

#[derive(Component)]
pub struct LevelState {
    l0: Entity,
    l1: Entity,
    l2: Entity,
    stage: LevelStage,
}

enum LevelStage {
    Booting { tt: f32, timer: Timer },
    AwaitingDialogCompletion { timer: Timer },
    Rebooting { tt: f32 },
}

pub fn process(
    time: Res<Time>,
    mut game_commands: EventWriter<Command>,
    mut level: Query<&mut LevelState>,
    mut lights: Query<&mut Light>,
    mut transforms: Query<&mut Transform>,
    mut typewriter_tx: EventWriter<TypewriterPrint>,
    mut change_hud_visibility_tx: EventWriter<ChangeHudVisibility>,
    mut goto_level_tx: EventWriter<GotoLevel>,
) {
    let Ok(mut level) = level.get_single_mut() else { return };
    let level = &mut *level;
    let level_lights = [level.l0, level.l1, level.l2];

    // -----

    for (light_idx, light) in level_lights.into_iter().enumerate() {
        let t = time.elapsed_seconds() + (light_idx as f32 * 2.0 * PI / 3.0);

        transforms.get_mut(light).unwrap().translation =
            Quat::from_rotation_y(t) * vec3(1.0, 0.0, 0.0) * 2.2
                + vec3(0.0, 1.5 + t.cos() * 0.8, 0.0);
    }

    // -----

    match &mut level.stage {
        LevelStage::Booting { tt, timer } => {
            if *tt == 0.0 {
                change_hud_visibility_tx.send(ChangeHudVisibility::hide());
            }

            *tt += time.delta_seconds();
            timer.tick(time.delta());

            for (light_idx, light) in level_lights.into_iter().enumerate() {
                lights.get_mut(light).unwrap().intensity =
                    ((*tt / 1.5) - (light_idx as f32 * 1.25)).min(0.8);
            }

            if timer.just_finished() {
                let dialogs = [
                    "yes, you're finally die-ing, yes !!",
                    "no, wait",
                    "uhm hmm, where are you ???",
                    "hmmmmmm",
                    "HEY WAIT, that's MY chillout room",
                    "how did you... nevermind",
                    "let me try again, stand still",
                ];

                for dialog in dialogs {
                    typewriter_tx.send(TypewriterPrint::new(dialog));
                }

                level.stage = LevelStage::AwaitingDialogCompletion {
                    timer: Timer::new(Duration::from_secs(30), TimerMode::Once),
                };
            }
        }

        LevelStage::AwaitingDialogCompletion { timer } => {
            timer.tick(time.delta());

            if timer.just_finished() {
                level.stage = LevelStage::Rebooting { tt: 0.0 };
            }
        }

        LevelStage::Rebooting { tt } => {
            *tt += time.delta_seconds();

            for light in level_lights {
                lights.get_mut(light).unwrap().intensity = (2.5 - *tt).max(0.0);
            }

            if *tt > 5.0 {
                game_commands.send(Command::SwitchTrack {
                    track: MusicTrack::Doome,
                });
                goto_level_tx.send(GotoLevel::new(Level::l5()));
            }
        }
    }
}
