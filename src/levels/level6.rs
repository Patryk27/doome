use std::f32::consts::PI;
use std::time::Duration;

use doome_bevy::convert::{graphical_to_physical, physical_to_graphical};

use crate::prelude::*;

pub fn init(
    mut commands: Commands,
    assets: Res<Assets>,
    mut goto_level_rx: EventReader<GotoLevel>,
    mut player: Query<(&mut Player, &mut Transform)>,
) {
    if !goto_level_rx.iter().any(|level| **level == Level::l6()) {
        return;
    }

    // -----

    let (mut player, mut player_xform) = player.single_mut();

    player.can_move = true;
    *player_xform = Default::default();

    // -----

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    let locator =
        LevelLoader::load(include_str!("../../assets/levels/level6.tmj"))
            .spawn(&mut lvl);

    lvl.point_light(
        locator.tag("light-1") + vec3(0.0, 1.8, 0.0),
        Color::hex(0xffffff),
        1.0,
    )
    .insert(Fade::fade_in(6.0));

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
    AwaitingPlayerToGoOutside,
    AwaitingFloorCreation1 {
        timer: Timer,
    },
    AwaitingFloorCreation2 {
        timer: Timer,
        floor: Entity,
    },
    AwaitingGeoDestroy {
        timer: Timer,
        rifle: Entity,
        floor: Entity,
    },
    AwaitingMinionsSpawn {
        floor: Entity,
        timer: Timer,
    },
    AwaitingMinionsDeath {
        floor: Entity,
        minions: Vec<Entity>,
    },
    AwaitingHealingUpSpawn {
        floor: Entity,
        timer: Timer,
    },
    AwaitingHealingUp {
        floor: Entity,
        hearts: Vec<Entity>,
        impatience: Timer,
    },
    AwaitingDialogEnd {
        floor: Entity,
    },
    AwaitingBossDeath {
        doome: Entity,
    },
    AwaitingOutro,
    Outro {
        texts: Vec<Entity>,
    },
}

pub fn process(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<Assets>,
    mut level: Query<&mut LevelState>,
    mut level_rx: EventReader<LevelGameplayEvent>,
    mut typewriter_tx: EventWriter<TypewriterPrint>,
    mut typewriter_rx: EventReader<TypewriterPrintingCompleted>,
    geos: Query<Entity, With<GeometryType>>,
    mut materials: Query<&mut Material>,
    mut sync_nav_data_tx: EventWriter<SyncNavData>,
    mut player: Query<(&mut Player, &Transform)>,
    mut inventory: Query<&mut Inventory>,
    mut change_hud_visibility_tx: EventWriter<ChangeHudVisibility>,
    mut transforms: Query<&mut Transform, Without<Player>>,
) {
    let Ok(mut level) = level.get_single_mut() else { return };
    let level = &mut *level;
    let dt = time.delta();

    match &mut level.stage {
        LevelStage::Intro => {
            typewriter_tx.send(TypewriterPrint::new(
                "aw yeah baby, we're back ON-LINE !!\nnow come",
            ));

            level.stage = LevelStage::AwaitingPlayerToGoOutside;
        }

        LevelStage::AwaitingPlayerToGoOutside => {
            let ready = level_rx.iter().any(|event| match event {
                LevelGameplayEvent::ZoneEntered(name) if name == "outside" => {
                    true
                }
                _ => false,
            });

            if !ready {
                return;
            }

            typewriter_tx.send(TypewriterPrint::new("hmm"));

            typewriter_tx.send(TypewriterPrint::new(
                "i feel like i forgot about something",
            ));

            typewriter_tx.send(TypewriterPrint::new("ah -- right!"));

            level.stage = LevelStage::AwaitingFloorCreation1 {
                timer: Timer::new(Duration::from_millis(7850), TimerMode::Once),
            };
        }

        LevelStage::AwaitingFloorCreation1 { timer } => {
            timer.tick(dt);

            if !timer.just_finished() {
                return;
            }

            typewriter_tx.send(TypewriterPrint::new("nope, almost"));

            let floor = LevelBuilder::new(&mut commands, &assets)
                .floor(-1000, -1000, 1000, 1000)
                .dynamic()
                .spawn()
                .id();

            level.stage = LevelStage::AwaitingFloorCreation2 {
                timer: Timer::new(Duration::from_secs(6), TimerMode::Once),
                floor,
            };
        }

        LevelStage::AwaitingFloorCreation2 { timer, floor } => {
            timer.tick(dt);

            if !timer.just_finished() {
                return;
            }

            typewriter_tx.send(TypewriterPrint::new("there, better"));
            typewriter_tx.send(TypewriterPrint::new("just one more touch..."));

            commands.entity(*floor).despawn();

            let rifle = Picker::rifle()
                .with_position(level.locator.tag("light-1").xz())
                .spawn(&assets, &mut commands);

            let floor = LevelBuilder::new(&mut commands, &assets)
                .floor(-1000, -1000, 1000, 1000)
                .dynamic()
                .alter_material(|mat| {
                    mat.with_color(Color::hex(0xffffff))
                        .with_reflectivity(0.95)
                        .with_reflection_color(Color::hex(0xffffff))
                        .without_texture()
                })
                .spawn()
                .id();

            inventory.single_mut().has_flashlight = true;

            level.stage = LevelStage::AwaitingGeoDestroy {
                timer: Timer::new(Duration::from_secs(6), TimerMode::Once),
                rifle,
                floor,
            };
        }

        LevelStage::AwaitingGeoDestroy {
            timer,
            rifle,
            floor,
        } => {
            timer.tick(dt);

            if !timer.just_finished() {
                return;
            }

            for geo in geos.iter() {
                if geo != *rifle && geo != *floor {
                    commands.entity(geo).despawn();
                }
            }

            // HACK our raytracer cries when there's no geometry whatsoever
            LevelBuilder::new(&mut commands, &assets)
                .floor(-1001, -1001, -1001, -1001)
                .spawn();

            sync_nav_data_tx.send(SyncNavData {
                force_aabb: Some((vec2(-100.0, -100.0), vec2(100.0, 100.0))),
            });

            typewriter_tx.send(TypewriterPrint::new("... and"));
            typewriter_tx.send(TypewriterPrint::new("MINIONS, ATTACK !"));

            level.stage = LevelStage::AwaitingMinionsSpawn {
                floor: *floor,
                timer: Timer::new(Duration::from_secs(6), TimerMode::Once),
            };
        }

        LevelStage::AwaitingMinionsSpawn { floor, timer } => {
            timer.tick(dt);

            if !timer.just_finished() {
                return;
            }

            let vs = ViewportSpawner::new(*player.single().1, 14.0, 3.5, 2);

            let minions = (0..2)
                .map(|minion_idx| {
                    let minion_pos = vs.pos(minion_idx);

                    MothMonster::spawn(
                        &assets,
                        &mut commands,
                        vec3(minion_pos.x, 0.0, minion_pos.y),
                    )
                })
                .collect();

            level.stage = LevelStage::AwaitingMinionsDeath {
                floor: *floor,
                minions,
            };
        }

        LevelStage::AwaitingMinionsDeath { floor, minions } => {
            let ready = minions
                .iter()
                .all(|minion| commands.get_entity(*minion).is_none());

            if !ready {
                return;
            }

            typewriter_tx.send(TypewriterPrint::new(
                "hm, i thought there would be more",
            ));

            typewriter_tx
                .send(TypewriterPrint::new("anyway, come on, heal up"));

            level.stage = LevelStage::AwaitingHealingUpSpawn {
                floor: *floor,
                timer: Timer::new(Duration::from_secs(4), TimerMode::Once),
            };
        }

        LevelStage::AwaitingHealingUpSpawn { floor, timer } => {
            timer.tick(dt);

            if !timer.just_finished() {
                return;
            }

            let vs = ViewportSpawner::new(*player.single().1, 8.0, 2.0, 10);

            let hearts = (0..10)
                .map(|heart_idx| {
                    Picker::heart()
                        .with_position(vs.pos(heart_idx))
                        .spawn(&assets, &mut commands)
                })
                .collect();

            level.stage = LevelStage::AwaitingHealingUp {
                floor: *floor,
                hearts,
                impatience: Timer::new(
                    Duration::from_secs(15),
                    TimerMode::Once,
                ),
            };
        }

        LevelStage::AwaitingHealingUp {
            floor,
            hearts,
            impatience,
        } => {
            impatience.tick(dt);

            if impatience.just_finished() {
                typewriter_tx.send(TypewriterPrint::new(
                    "come on, i don't have the entire day",
                ));
            }

            let ready = hearts
                .iter()
                .all(|heart| commands.get_entity(*heart).is_none());

            if !ready {
                return;
            }

            let dialogs = [
                "alright",
                "you've proven yourself to be a worthy\nopponent",
                "now it's high time we finally meet",
                "but before, let me explain you my motives",
                "you see",
                "i want everybody to die simply because i am\nevil",
            ];

            for dialog in dialogs {
                typewriter_tx.send(TypewriterPrint::new(dialog));
            }

            typewriter_tx.send(
                TypewriterPrint::new("thank you for coming to my ted talk")
                    .with_id("last-dialog"),
            );

            level.stage = LevelStage::AwaitingDialogEnd { floor: *floor };
        }

        LevelStage::AwaitingDialogEnd { floor } => {
            let mut floor_mat = materials.get_mut(*floor).unwrap();
            let mut floor_color = floor_mat.color.as_mut().unwrap();

            if floor_color.r > 0.0 {
                floor_color.r =
                    (floor_color.r - time.delta_seconds() / 15.0).max(0.0);

                floor_color.g = floor_color.r;
                floor_color.b = floor_color.r;
            }

            let ready =
                typewriter_rx.iter().any(|event| event.id == "last-dialog");

            if !ready {
                return;
            }

            let doome_pos =
                ViewportSpawner::new(*player.single().1, 40.0, 1.0, 1).pos(0);

            let doome = {
                Doome::spawn(
                    &assets,
                    &mut commands,
                    vec3(doome_pos.x, 0.0, doome_pos.y),
                )
            };

            let doome_pos = physical_to_graphical(doome_pos);

            const DISTANCE_TO_BOSS: f32 = 30.0;
            const NUM_COVERS: usize = 16;

            let angle = 2.0 * PI / NUM_COVERS as f32;

            for cover_idx in 0..NUM_COVERS {
                let angle = angle * cover_idx as f32;
                let rotation = Quat::from_rotation_y(angle);

                let mut transform = Transform::from_rotation(rotation);
                let offset = transform.forward() * DISTANCE_TO_BOSS;

                transform.translation = doome_pos + offset;

                for r in 0..4 {
                    let angle =
                        angle + (r as f32 * PI / 2.0) + (3.0 * PI / 2.0);

                    let rotation = Quat::from_rotation_y(angle);

                    let mut transform = transform
                        .clone()
                        .with_translation(transform.translation)
                        .with_rotation(rotation);

                    transform.translation += transform.forward();

                    let mut lvl = LevelBuilder::new(&mut commands, &assets);

                    lvl.model("wall")
                        .with_transform(transform)
                        .obstacle()
                        .with_material(
                            Material::default()
                                .with_color(Color::hex(0xffffffff))
                                .emissive(),
                        )
                        .with_collider(Collider::line(
                            vec2(-1.0, 0.0),
                            vec2(1.0, 0.0),
                        ))
                        .spawn();
                }

                Picker::rpg()
                    .with_position(graphical_to_physical(
                        transform.translation + transform.forward() * 2.0,
                    ))
                    .spawn(&assets, &mut commands);
            }

            level.stage = LevelStage::AwaitingBossDeath { doome };
        }

        LevelStage::AwaitingBossDeath { doome } => {
            let ready = commands.get_entity(*doome).is_none();

            if !ready {
                return;
            }

            let dialogs = [
                "oh no",
                "oh my",
                "that's awkward",
                "so, uhm, i guess, i am dead now?",
                "IN YOUR DREAMS",
                "i don't have any physical form anymore, tho",
                "so there's that",
                "you know what",
                "let's just say you won",
                "YOU WON",
                "player AAA: highscore: 100kk",
                "happy?",
            ];

            for dialog in dialogs {
                typewriter_tx.send(TypewriterPrint::new(dialog));
            }

            typewriter_tx.send(
                TypewriterPrint::new(
                    "anyway... until we meet again\nand we *will* meet again",
                )
                .with_id("last-dialog"),
            );

            change_hud_visibility_tx.send(ChangeHudVisibility::hide());

            level.stage = LevelStage::AwaitingOutro;
        }

        LevelStage::AwaitingOutro => {
            let ready =
                typewriter_rx.iter().any(|event| event.id == "last-dialog");

            if !ready {
                return;
            }

            player.single_mut().0.can_move = false;

            for geo in geos.iter() {
                commands.entity(geo).despawn();
            }

            // HACK our raytracer cries when there's no geometry whatsoever
            LevelBuilder::new(&mut commands, &assets)
                .floor(-1001, -1001, -1001, -1001)
                .spawn();

            let texts = [
                "Doome",
                "",
                "CAST",
                "Moth Monster",
                "The Boss",
                "",
                "PRODUCED BY",
                "dzejkop (Jakub Trad)",
                "Patryk27 (Patryk Wychowaniec)",
                "",
                "LIGHTS, MATERIALS, CAMERAS",
                ".sin(), .cos(), .tan()",
                "",
                "BASED ON THE NOVEL BY",
                "Id Software",
                "",
                "SPECIAL THANKS",
                "coffee, yerba mate, idk",
                "",
                "LINKS",
                "https://dzejkop.itch.io/doome",
                "https://github.com/patryk27/doome",
                "",
                "Thanks for playing!",
            ];

            level.stage = LevelStage::Outro {
                texts: texts
                    .into_iter()
                    .enumerate()
                    .map(|(text_idx, text)| {
                        commands
                            .spawn((
                                Text::new(text).centered(),
                                Transform::from_translation(vec3(
                                    0.5,
                                    1.1 + (text_idx as f32 / 10.0),
                                    0.0,
                                )),
                                Visibility::visible(),
                            ))
                            .id()
                    })
                    .collect(),
            };
        }

        LevelStage::Outro { texts } => {
            for text in texts {
                transforms.get_mut(*text).unwrap().translation.y -=
                    time.delta_seconds() / 8.5;
            }
        }
    }
}

struct ViewportSpawner {
    player_xform: Transform,
    distance: f32,
    item_size: f32,
    item_count: usize,
}

impl ViewportSpawner {
    fn new(
        player_xform: Transform,
        distance: f32,
        item_size: f32,
        item_count: usize,
    ) -> Self {
        Self {
            player_xform,
            distance,
            item_size,
            item_count,
        }
    }

    fn pos(&self, item_idx: usize) -> Vec2 {
        let pos = self.player_xform.translation
            + self.player_xform.forward() * self.distance;

        let dir = self.player_xform.forward().xz().perp().normalize()
            * self.item_size;

        pos.xz() - dir * ((self.item_count / 2) as f32)
            + dir * (item_idx as f32)
    }
}
