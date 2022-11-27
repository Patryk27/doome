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
    SpawningWaveEnemies { delay: Timer, wave: u8 },
    AwaitingWaveCompletion { enemies: Vec<Entity>, wave: u8 },
    PostWaveCooldown { cooldown: Timer, next_wave: u8 },
    AwaitingGoingThroughDoor,
    AwaitingLeaving { light: Entity },
    AwaitingOutro { timer: Timer },
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
    mut inventory: Query<&mut Inventory>,
    mut goto_level_tx: EventWriter<GotoLevel>,
    mut player: Query<&mut Player>,
) {
    let Ok(mut level) = level.get_single_mut() else { return };
    let level = &mut *level;
    let dt = time.delta();

    match &mut level.stage {
        LevelStage::Intro => {
            typewriter_tx.send(TypewriterPrint::new(
                "yes, now it's correct -- DIE.....",
            ));

            sync_nav_data_tx.send(SyncNavData::default());

            level.stage = LevelStage::SpawningWave { wave: 1 };
        }

        LevelStage::SpawningWave { wave } => {
            let prev_wave_name = format!("wave{}", *wave - 1);
            let wave_name = format!("wave{}", *wave);

            for (torch_name, torch_entity) in level.locator.torches() {
                if torch_name.starts_with(&prev_wave_name) {
                    commands.entity(torch_entity).remove::<TorchActive>();
                } else if torch_name.starts_with(&wave_name) {
                    commands
                        .entity(torch_entity)
                        .insert(TorchActive::in_ms(1000));
                }
            }

            level.stage = LevelStage::SpawningWaveEnemies {
                delay: Timer::new(Duration::from_secs(1), TimerMode::Once),
                wave: *wave,
            };
        }

        LevelStage::SpawningWaveEnemies { delay, wave } => {
            delay.tick(dt);

            if !delay.just_finished() {
                return;
            }

            let mut enemies = Vec::new();

            for (tag_name, tag_position) in level.locator.tags() {
                if tag_name.starts_with(&format!("wave{}.spawn", *wave)) {
                    enemies.push(MothMonster::spawn(
                        &assets,
                        &mut commands,
                        tag_position,
                    ));
                }
            }

            level.stage = LevelStage::AwaitingWaveCompletion {
                enemies,
                wave: *wave,
            };
        }

        LevelStage::AwaitingWaveCompletion { enemies, wave } => {
            let are_enemies_still_alive = enemies
                .iter()
                .any(|enemy| commands.get_entity(*enemy).is_some());

            if are_enemies_still_alive {
                return;
            }

            match *wave {
                1 => {
                    typewriter_tx.send(TypewriterPrint::new(
                        "ouch.... no worries though, I've got more!!\n\
                         just give me a second",
                    ));

                    Picker::rifle()
                        .with_position(level.locator.tag("wave1.loot").xz())
                        .spawn(&assets, &mut commands);

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

                    level.stage = LevelStage::AwaitingGoingThroughDoor;
                }

                _ => unreachable!(),
            }
        }

        LevelStage::PostWaveCooldown {
            cooldown,
            next_wave,
        } => {
            cooldown.tick(dt);

            if cooldown.just_finished() {
                typewriter_tx.send(TypewriterPrint::new("a ha! -- DIE....."));

                level.stage = LevelStage::SpawningWave { wave: *next_wave };
            }
        }

        LevelStage::AwaitingGoingThroughDoor => {
            for event in level_rx.iter() {
                match event {
                    LevelGameplayEvent::DoorOpened(name) => {
                        match name.as_str() {
                            "3" => {
                                typewriter_tx.send(TypewriterPrint::new(
                                    "whoops, looks like your flashlight broke down\n\
                                     fortunately, the end is near -- follow the light!",
                                ));

                                inventory.single_mut().has_flashlight = false;

                                let light =
                                    LevelBuilder::new(&mut commands, &assets)
                                        .point_light(
                                            level.locator.tag("exit-light")
                                                + vec3(0.0, 1.5, 0.0),
                                            Color::hex(0xffffff),
                                            1.0,
                                        )
                                        .id();

                                level.stage =
                                    LevelStage::AwaitingLeaving { light };
                            }

                            _ => {
                                //
                            }
                        }
                    }

                    LevelGameplayEvent::KeyPicked(name) => {
                        let inventory = inventory.single();

                        if name == "2" || name == "3" {
                            match inventory.keys.len() {
                                2 => {
                                    typewriter_tx.send(TypewriterPrint::new(
                                        "gotcha!! yes... YES......",
                                    ));
                                }

                                3 => {
                                    typewriter_tx.send(TypewriterPrint::new(
                                        "oh well, come see me....",
                                    ));
                                }

                                _ => {
                                    //
                                }
                            }

                            MothMonster::spawn(
                                &assets,
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

        LevelStage::AwaitingLeaving { light } => {
            let mut ready = false;

            for event in level_rx.iter() {
                match event {
                    LevelGameplayEvent::ZoneEntered(name) => {
                        if name == "end" {
                            ready = true;
                        }
                    }

                    _ => {
                        //
                    }
                }
            }

            if ready {
                commands.entity(*light).despawn();

                typewriter_tx.send(TypewriterPrint::new(
                    "technical problems, SORRY\nfirst time killing somebody",
                ));

                player.single_mut().can_move = false;

                level.stage = LevelStage::AwaitingOutro {
                    timer: Timer::new(Duration::from_secs(8), TimerMode::Once),
                };
            }
        }

        LevelStage::AwaitingOutro { timer } => {
            timer.tick(dt);

            if timer.just_finished() {
                goto_level_tx.send(GotoLevel::new(Level::l6()));
            }
        }
    }
}
