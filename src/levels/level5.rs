use std::time::Duration;

use crate::prelude::*;

pub fn init(
    mut commands: Commands,
    assets: Res<Assets>,
    mut goto_level_rx: EventReader<GotoLevel>,
    mut player: Query<(&mut Player, &mut Transform)>,
) {
    if !goto_level_rx.iter().any(|level| **level == Level::l5()) {
        return;
    }

    // -----

    let (mut player, mut player_xform) = player.single_mut();

    player.can_move = true;
    *player_xform = Transform::default().with_translation(vec3(0.0, 0.0, 9.0));

    // -----

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    let locator =
        LevelLoader::load(include_str!("../../assets/levels/level5.tmj"))
            .spawn(&mut lvl);

    // -----

    lvl.complete(LevelState {
        locator,
        stage: LevelStage::Intro,
    });
}

#[derive(Component)]
pub struct LevelState {
    locator: LevelLocator,
    stage: LevelStage,
}

enum LevelStage {
    Intro,
    SpawningWave { wave: u8 },
    AwaitingWaveCompletion { monsters: Vec<Entity>, wave: u8 },
    PostWaveCooldown { cooldown: Timer, next_wave: u8 },
    AwaitingLeaving,
}

pub fn process(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<Assets>,
    mut level: Query<&mut LevelState>,
    mut typewriter_tx: EventWriter<TypewriterPrint>,
    mut sync_nav_data_tx: EventWriter<SyncNavData>,
    mut command_tx: EventWriter<Command>,
    mut level_rx: EventReader<LevelGameplayEvent>,
    inventory: Query<&Inventory>,
) {
    let Ok(mut level) = level.get_single_mut() else { return };
    let level = &mut *level;

    match &mut level.stage {
        LevelStage::Intro => {
            typewriter_tx.send(TypewriterPrint::new(
                "yes, now it's correct -- DIE.....",
            ));

            sync_nav_data_tx.send(SyncNavData);

            level.stage = LevelStage::SpawningWave { wave: 1 };
        }

        LevelStage::SpawningWave { wave } => {
            let prev_wave_name = format!("wave{}", *wave - 1);
            let wave_name = format!("wave{}", *wave);
            let mut monsters = Vec::new();

            for (torch_name, torch_entity) in level.locator.torches() {
                if torch_name.starts_with(&prev_wave_name) {
                    commands.entity(torch_entity).remove::<TorchActive>();
                } else if torch_name.starts_with(&wave_name) {
                    commands
                        .entity(torch_entity)
                        .insert(TorchActive::in_ms(1000));
                }
            }

            for (tag_name, tag_position) in level.locator.tags() {
                if tag_name.starts_with(&wave_name) {
                    monsters.push(MothMonster::spawn(
                        &*assets,
                        &mut commands,
                        tag_position,
                    ));
                }
            }

            level.stage = LevelStage::AwaitingWaveCompletion {
                monsters,
                wave: *wave,
            };
        }

        LevelStage::AwaitingWaveCompletion { monsters, wave } => {
            let are_monsters_still_alive = monsters
                .iter()
                .any(|monster| commands.get_entity(*monster).is_some());

            if are_monsters_still_alive {
                return;
            }

            match *wave {
                1 => {
                    typewriter_tx.send(TypewriterPrint::new(
                        "ouch.... no worries though, I've got more!!\n\
                         just give me a second",
                    ));

                    level.stage = LevelStage::PostWaveCooldown {
                        cooldown: Timer::new(
                            Duration::from_secs(10),
                            TimerMode::Once,
                        ),
                        next_wave: 2,
                    };
                }

                2 => {
                    typewriter_tx.send(TypewriterPrint::new(
                        "hmm that's not going as i expected.....",
                    ));

                    level.stage = LevelStage::PostWaveCooldown {
                        cooldown: Timer::new(
                            Duration::from_secs(5),
                            TimerMode::Once,
                        ),
                        next_wave: 3,
                    };
                }

                3 => {
                    typewriter_tx.send(TypewriterPrint::new(
                        "nah i'm bored\n\
                         here, take this flashlight and go explore....\n\
                         it is safe.... you can trust me..... ..... !!",
                    ));

                    for (_, torch_entity) in level.locator.torches() {
                        commands.entity(torch_entity).remove::<TorchActive>();
                    }

                    command_tx.send(Command::Give {
                        what: Item::Flashlight,
                    });

                    level.stage = LevelStage::AwaitingLeaving;
                }

                _ => unreachable!(),
            }
        }

        LevelStage::PostWaveCooldown {
            cooldown,
            next_wave,
        } => {
            cooldown.tick(time.delta());

            if cooldown.just_finished() {
                typewriter_tx.send(TypewriterPrint::new("a ha! -- DIE....."));

                level.stage = LevelStage::SpawningWave { wave: *next_wave };
            }
        }

        LevelStage::AwaitingLeaving => {
            for event in level_rx.iter() {
                match event {
                    LevelGameplayEvent::DoorOpened(name) => {
                        // TODO
                    }

                    LevelGameplayEvent::KeyPicked(name) => {
                        let inventory = inventory.single();

                        if name == "2" || name == "3" {
                            if inventory.keys.len() == 2 {
                                typewriter_tx.send(TypewriterPrint::new(
                                    "gotcha!! yes... YES......",
                                ));
                            } else {
                                typewriter_tx.send(TypewriterPrint::new(
                                    "oh well, come see me....",
                                ));
                            }

                            MothMonster::spawn(
                                &*assets,
                                &mut commands,
                                level.locator.tag(if name == "2" {
                                    "room-a.spawn"
                                } else {
                                    "room-b.spawn"
                                }),
                            );
                        }
                    }

                    _ => {
                        //
                    }
                }
            }
        }
    }
}
