use std::f32::consts::PI;
use std::time::Duration;

use super::builder::LevelBuilder;
use super::loader::{LevelLoader, LevelLocator};
use crate::inventory::Inventory;
use crate::prelude::*;

pub fn init(
    mut commands: Commands,
    assets: Res<Assets>,
    mut goto_level_rx: EventReader<GotoLevel>,
    mut player: Query<(&mut Player, &mut Transform)>,
) {
    if !goto_level_rx.iter().any(|level| **level == Level::l2()) {
        return;
    }

    // -----

    let (mut player, mut player_xform) = player.single_mut();

    player.can_move = true;

    *player_xform = Transform::default()
        .with_translation(vec3(0.0, 0.0, -8.0))
        .with_rotation(Quat::from_rotation_y(PI));

    // -----

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    lvl.model("ceiling")
        .with_translation(vec3(0.0, 1.25 * 10.0, 0.0))
        .with_scale(vec3(15.0, 1.0, 15.0))
        .with_rotation(Quat::from_rotation_y(PI / 1.5))
        .with_material(
            Material::default()
                .with_color(Color::hex(0xffffff))
                .with_texture(assets.load_texture("wall.stone"))
                .with_uv_divisor(6, 6)
                .without_casting_shadows(),
        )
        .spawn();

    lvl.model("floor")
        .with_translation(vec3(1.0, 0.1, 0.0))
        .with_scale(vec3(10.0, 1.0, 10.0))
        .with_material(
            Material::default()
                .with_color(Color::hex(0xffffff) * 0.75)
                .with_reflectivity(0.25)
                .with_reflection_color(Color::hex(0xffffff))
                .with_texture(assets.load_texture("floor.stone.mossy.water"))
                .without_casting_shadows()
                .with_uv_divisor(8, 8),
        )
        .spawn();

    lvl.model("table")
        .with_translation(vec3(0.0, 0.0, 0.0))
        .with_rotation(Quat::from_rotation_y(-PI / 2.0))
        .with_scale(Vec3::ONE * 0.6)
        .with_material(Material::default().with_uv_divisor(4, 4))
        .with_collider(Collider::line(vec2(-1.0, 0.95), vec2(1.0, 0.95)))
        .spawn();

    for n in 0..6 {
        let nf = n as f32 / 3.0 * PI;
        let pos = vec3(-8.5 * nf.cos(), 0.0, 8.5 * nf.sin());

        if n == 0 {
            for z in [-5.0, 5.0] {
                lvl.model("wall")
                    .with_translation(pos + vec3(0.0, 0.0, z))
                    .with_rotation(Quat::from_rotation_y(nf - PI / 2.0))
                    .with_scale(vec3(1.5, 1.25 * 5.0, 1.0))
                    .with_material(
                        Material::default()
                            .with_color(Color::hex(0xffffff))
                            .with_texture(assets.load_texture("wall.stone"))
                            .with_uv_divisor(2, 5),
                    )
                    .with_collider(Collider::line(
                        vec2(-1.0, 0.0),
                        vec2(1.0, 0.0),
                    ))
                    .spawn();
            }

            lvl.model("wall")
                .with_translation(pos + vec3(0.0, 2.5, 0.0))
                .with_rotation(Quat::from_rotation_y(nf - PI / 2.0))
                .with_scale(vec3(4.0, 1.25 * 5.0, 1.0))
                .with_material(
                    Material::default()
                        .with_color(Color::hex(0xffffff))
                        .with_texture(assets.load_texture("wall.stone"))
                        .with_uv_divisor(4, 5),
                )
                .spawn();
        }

        if n == 2 {
            lvl.model("floor") // hehe
                .with_translation(pos + vec3(-0.2, 2.5, -0.2))
                .with_scale(vec3(1.0, 1.0, 1.0))
                .with_rotation(
                    Quat::from_rotation_x(-PI / 2.0)
                        * Quat::from_rotation_z(PI / 6.0),
                )
                .with_material(
                    Material::default()
                        .with_color(Color::hex(0xffffff))
                        .with_texture(assets.load_texture("text.die"))
                        .without_casting_shadows()
                        .with_uv_transparency(),
                )
                .spawn();
        }

        if n == 3 {
            lvl.model("grate")
                .with_translation(pos + vec3(-0.05, 1.0, 0.0))
                .with_rotation(Quat::from_rotation_y(nf))
                .with_scale(vec3(5.0, 10.0, 5.0))
                .with_material(
                    Material::default()
                        .with_color(Color::hex(0xffffff))
                        .with_uv_transparency()
                        .without_casting_shadows(),
                )
                .spawn();
        }

        if n > 0 {
            lvl.model("wall")
                .obstacle()
                .with_translation(pos)
                .with_rotation(Quat::from_rotation_y(nf - PI / 2.0))
                .with_scale(vec3(6.0, 1.25 * 5.0, 1.0))
                .with_material(
                    Material::default()
                        .with_color(Color::hex(0xffffff))
                        .with_texture(assets.load_texture("wall.stone"))
                        .without_casting_shadows()
                        .with_uv_divisor(8, 5),
                )
                .with_collider(Collider::line(vec2(-1.0, 0.0), vec2(1.0, 0.0)))
                .spawn();
        }
    }

    let ent_gate = lvl
        .model("gate")
        .dynamic()
        .with_translation(vec3(-8.52, 0.0, 0.0))
        .with_scale(vec3(1.0, 1.5, 4.0))
        .with_material(
            Material::default()
                .with_color(Color::hex(0xffffff) * 0.75)
                .with_uv_divisor(3, 1)
                .with_uv_transparency(),
        )
        .with_collider(Collider::line(vec2(0.0, -1.0), vec2(0.0, 1.0)))
        .spawn()
        .id();

    let ent_lamp = lvl
        .model("lamp.metal")
        .dynamic()
        .with_translation(vec3(0.0, 8.0, 0.0))
        .with_material(
            Material::default()
                .emissive()
                .with_color(Color::hex(0xffffff) * 0.05)
                .without_casting_shadows(),
        )
        .spawn()
        .id();

    let ent_sl0 = lvl
        .spot_light(
            vec3(0.0, 8.0, 0.0),
            vec3(0.0, 0.0, 0.0),
            PI / 3.0,
            Color::hex(0xffffff) * 0.75,
            0.0,
        )
        .insert(LightFade::fade_in(1.5))
        .id();

    let ent_l0 = lvl
        .point_light(vec3(-15.0, 2.0, 0.0), Color::hex(0xff0000), 0.0)
        .id();

    let ent_flashlight = FlashlightPicker::spawn(
        lvl.assets(),
        lvl.commands(),
        vec3(-6.8, 0.0, 0.5),
    );

    // -----

    let locator =
        LevelLoader::load(include_str!("../../assets/levels/level2.tmj"))
            .spawn(&mut lvl);

    lvl.model("gate")
        .dynamic()
        .with_translation(locator.world_point("gate.2"))
        .with_scale(vec3(1.0, 1.0, 3.0))
        .with_material(
            Material::default()
                .two_sided()
                .with_color(Color::hex(0xffffff) * 0.75)
                .with_uv_divisor(3, 1)
                .with_uv_transparency(),
        )
        .with_collider(Collider::line(vec2(0.0, -1.0), vec2(0.0, 1.0)))
        .spawn();

    // -----

    lvl.complete(LevelState {
        locator,
        ent_gate,
        ent_lamp,
        ent_sl0,
        ent_l0,
        ent_flashlight,
        stage: LevelStage::AwaitingFlashlightPickup,
    });
}

#[derive(Component)]
pub struct LevelState {
    locator: LevelLocator,
    ent_gate: Entity,
    ent_lamp: Entity,
    ent_sl0: Entity,
    ent_l0: Entity,
    ent_flashlight: Entity,
    stage: LevelStage,
}

enum LevelStage {
    AwaitingFlashlightPickup,
    Intro0 { txt_rise_timer: Timer },
    Intro1 { spawn_timer: Timer },
    AwaitingMothsDeath { moths: Vec<Entity> },
    AwaitingZoneEnter,
    AwaitingKeyPickup,
}

pub fn process(
    time: Res<Time>,
    assets: Res<Assets>,
    mut commands: Commands,
    mut level: Query<&mut LevelState>,
    mut transforms: Query<&mut Transform>,
    mut lights: Query<&mut Light>,
    mut typewriter_tx: EventWriter<TypewriterPrint>,
    mut message_tx: EventWriter<Message>,
    mut recalc_nav_data_tx: EventWriter<SyncNavData>,
    mut level_tx: EventReader<LevelGameplayEvent>,
    inventory: Query<&Inventory>,
) {
    let Ok(mut level) = level.get_single_mut() else { return };
    let t = time.elapsed_seconds();
    let dt = time.delta();
    let level = &mut *level;

    // -----

    if let Ok(mut light) = lights.get_mut(level.ent_sl0) {
        light.point_at_mut().unwrap().x = (1.8 * t).sin() * 1.8;
    }

    if let Ok(mut lamp) = transforms.get_mut(level.ent_lamp) {
        lamp.rotation = Quat::from_rotation_z((1.8 * t).sin() / 2.0);
    }

    // -----

    match &mut level.stage {
        LevelStage::AwaitingFlashlightPickup => {
            let Ok(inventory) = inventory.get_single() else { return };

            if inventory.has_flashlight {
                level.stage = LevelStage::Intro0 {
                    txt_rise_timer: Timer::new(
                        Duration::from_secs(5),
                        TimerMode::Once,
                    ),
                };
            }
        }

        LevelStage::Intro0 { txt_rise_timer } => {
            txt_rise_timer.tick(dt);

            if txt_rise_timer.just_finished() {
                typewriter_tx.send(TypewriterPrint::new(
                    "rise and shine mr freeman....... rise and shine....",
                ));

                level.stage = LevelStage::Intro1 {
                    spawn_timer: Timer::new(
                        Duration::from_millis(3150),
                        TimerMode::Once,
                    ),
                };
            }
        }

        LevelStage::Intro1 { spawn_timer } => {
            spawn_timer.tick(dt);

            transforms.get_mut(level.ent_gate).unwrap().translation.y +=
                time.delta_seconds() / 1.2;

            if spawn_timer.just_finished() {
                commands.entity(level.ent_gate).remove::<Collider>();

                commands
                    .entity(level.ent_sl0)
                    .insert(LightFade::fade_out(0.35));

                commands
                    .entity(level.ent_l0)
                    .insert(LightFade::fade_in(0.35));

                commands.entity(level.ent_lamp).despawn();

                recalc_nav_data_tx.send(SyncNavData);

                let moths = (1..5).map(|id| {
                    MothMonster::spawn(
                        &assets,
                        &mut commands,
                        level.locator.world_point(&format!("monster.{}", id)),
                    )
                });

                level.stage = LevelStage::AwaitingMothsDeath {
                    moths: moths.collect(),
                };
            }
        }

        LevelStage::AwaitingMothsDeath { moths } => {
            for moth in moths {
                if commands.get_entity(*moth).is_some() {
                    return;
                }
            }

            commands
                .entity(level.ent_l0)
                .insert(LightFade::fade_out(4.0));

            typewriter_tx.send(TypewriterPrint::new(
                "ouch... oh no........ i'll see you again..... soon........",
            ));

            level.stage = LevelStage::AwaitingZoneEnter;
        }

        LevelStage::AwaitingZoneEnter => {
            for event in level_tx.iter() {
                match event {
                    LevelGameplayEvent::ZoneEntered(name)
                        if name == "zone.door.1" || name == "zone.door.2" =>
                    {
                        typewriter_tx.send(TypewriterPrint::new(
                            "gotcha this time......",
                        ));

                        let (moth_spawn_point, other_door) =
                            if name == "zone.door.1" {
                                ("monster.door.1", "door.v.2")
                            } else {
                                ("monster.door.2", "door.v.1")
                            };

                        for sp in [moth_spawn_point, "monster.door.3"] {
                            MothMonster::spawn(
                                &assets,
                                &mut commands,
                                level.locator.world_point(sp),
                            );
                        }

                        commands
                            .entity(level.locator.entity(other_door))
                            .despawn();

                        level.stage = LevelStage::AwaitingKeyPickup;
                    }

                    _ => {
                        //
                    }
                }
            }
        }

        LevelStage::AwaitingKeyPickup => {
            //
        }
    }
}
