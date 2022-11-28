use std::time::Duration;

use crate::prelude::*;

pub fn init(
    mut commands: Commands,
    assets: Res<Assets>,
    mut goto_level_rx: EventReader<GotoLevel>,
    mut player: Query<(&mut Player, &mut Transform)>,
) {
    if !goto_level_rx.iter().any(|level| **level == Level::l3()) {
        return;
    }

    // -----

    let (mut player, mut player_xform) = player.single_mut();

    player.can_move = true;
    *player_xform = Transform::default();

    // -----

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    let locator =
        LevelLoader::load(include_str!("../../assets/levels/level3.tmj"))
            .spawn(&mut lvl);

    // -----

    lvl.complete(LevelState {
        locator,
        stage: LevelStage::EntryIntro,
    });
}

#[derive(Component)]
pub struct LevelState {
    locator: LevelLocator,
    stage: LevelStage,
}

enum LevelStage {
    EntryIntro,
    EntryAwaitingKey,
    EntryAwaitingDoor {
        corridor: &'static str,
    },
    InsideCorridor {
        dialog1_said: bool,
        dialog2_said: bool,
    },
    Trapped {
        timer: Timer,
    },
    Outro {
        timer: Timer,
    },
}

pub fn process(
    time: Res<Time>,
    mut commands: Commands,
    mut level: Query<&mut LevelState>,
    mut typewriter_tx: EventWriter<TypewriterPrint>,
    mut level_rx: EventReader<LevelGameplayEvent>,
    mut player: Query<(&mut Player, &mut Transform)>,
    transforms: Query<&Transform, Without<Player>>,
    torch_actives: Query<&TorchActive>,
    mut goto_level_tx: EventWriter<GotoLevel>,
) {
    let Ok(mut level) = level.get_single_mut() else { return };
    let level = &mut *level;
    let dt = time.delta();

    // -----

    match &mut level.stage {
        LevelStage::EntryIntro => {
            typewriter_tx.send(TypewriterPrint::new(
                "illusion is just a choice, mr anderson......",
            ));

            level.stage = LevelStage::EntryAwaitingKey;
        }

        LevelStage::EntryAwaitingKey => {
            let mut corridor = None;

            for event in level_rx.iter() {
                match event {
                    LevelGameplayEvent::KeyPicked(key_name) => {
                        for corridor in ["a", "b", "c"] {
                            let key = level
                                .locator
                                .key(format!("corridor-{}.entry", corridor));

                            if let Some(mut key) = commands.get_entity(key) {
                                key.despawn();
                            }
                        }

                        corridor = Some(match key_name.as_str() {
                            "corridor-a.entry" => "a",
                            "corridor-b.entry" => "b",
                            "corridor-c.entry" => "c",
                            _ => unreachable!(),
                        });
                    }

                    LevelGameplayEvent::ZoneEntered(zone_name) => {
                        if zone_name == "accidental-exit" {
                            typewriter_tx.send(TypewriterPrint::new(
                                "no ! -- there's nothing for you there",
                            ));

                            player.single_mut().1.translation =
                                Default::default();
                        }
                    }

                    _ => {
                        //
                    }
                }
            }

            let Some(corridor) = corridor else { return };

            level.stage = LevelStage::EntryAwaitingDoor { corridor };
        }

        LevelStage::EntryAwaitingDoor { corridor } => {
            let mut ready = false;

            for event in level_rx.iter() {
                match event {
                    LevelGameplayEvent::DoorOpened(_) => {
                        ready = true;
                    }

                    LevelGameplayEvent::ZoneEntered(zone_name) => {
                        if zone_name == "accidental-exit" {
                            typewriter_tx.send(TypewriterPrint::new(
                                "no ! -- there's nothing for you there",
                            ));

                            player.single_mut().1.translation =
                                Default::default();
                        }
                    }

                    _ => {
                        //
                    }
                }
            }

            if !ready {
                return;
            }

            typewriter_tx.send(TypewriterPrint::new(
                "you want to fight ??! -- let's fight!",
            ));

            commands
                .entity(level.locator.torch("entry-1"))
                .remove::<TorchActive>();

            commands
                .entity(level.locator.torch("entry-2"))
                .remove::<TorchActive>();

            commands
                .entity(level.locator.torch(format!("corridor-{}", corridor)))
                .insert(TorchActive::now());

            level.stage = LevelStage::InsideCorridor {
                dialog1_said: false,
                dialog2_said: false,
            };
        }

        LevelStage::InsideCorridor {
            dialog1_said,
            dialog2_said,
        } => {
            let mut ready = false;

            for event in level_rx.iter() {
                match event {
                    LevelGameplayEvent::ZoneEntered(zone_name) => {
                        match zone_name.as_str() {
                            "corridor.dialog-1" => {
                                if !*dialog1_said {
                                    typewriter_tx.send(TypewriterPrint::new(
                                        "uhm hey what are you doing",
                                    ));

                                    *dialog1_said = true;
                                }
                            }

                            "corridor.dialog-2" => {
                                if !*dialog2_said {
                                    typewriter_tx.send(TypewriterPrint::new(
                                        "come on i was just joking please hey",
                                    ));

                                    *dialog2_said = true;
                                }
                            }

                            "trap" => {
                                ready = true;
                            }

                            _ => {
                                //
                            }
                        }
                    }

                    _ => {
                        //
                    }
                }
            }

            // -----

            let player_xz = player.single().1.translation.xz();

            let dynamic_torches = level
                .locator
                .torches()
                .filter(|(torch_name, _)| torch_name.starts_with("d"));

            for (torch_name, torch_entity) in dynamic_torches {
                let torch_xform = transforms.get(torch_entity).unwrap();

                if ready {
                    if torch_name != "d9" {
                        commands.entity(torch_entity).remove::<TorchActive>();
                    }
                } else {
                    if player_xz.distance(torch_xform.translation.xz()) < 7.5 {
                        if torch_actives.get(torch_entity).is_err() {
                            commands
                                .entity(torch_entity)
                                .insert(TorchActive::now());
                        }
                    } else {
                        if torch_actives.get(torch_entity).is_ok() {
                            commands
                                .entity(torch_entity)
                                .remove::<TorchActive>();
                        }
                    }
                }
            }

            // -----

            if ready {
                typewriter_tx.send(TypewriterPrint::new(
                    "ha, ha! -- just as i planned !!! prepare to D-I-E...",
                ));

                player.single_mut().0.can_move = false;

                level.stage = LevelStage::Trapped {
                    timer: Timer::new(Duration::from_secs(8), TimerMode::Once),
                };
            }
        }

        LevelStage::Trapped { timer } => {
            timer.tick(dt);

            if timer.just_finished() {
                commands
                    .entity(level.locator.torch("d9"))
                    .remove::<TorchActive>();

                level.stage = LevelStage::Outro {
                    timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
                };
            }
        }

        LevelStage::Outro { timer } => {
            timer.tick(dt);

            if timer.just_finished() {
                goto_level_tx.send(GotoLevel::new(Level::l4()));
            }
        }
    }
}
