use std::ops;
use std::str::FromStr;

use crate::prelude::*;

pub struct LevelsCoordinator;

impl LevelsCoordinator {
    pub fn init(mut goto_level_tx: EventWriter<GotoLevel>) {
        goto_level_tx.send(GotoLevel::new(Level::l2()));
    }

    pub fn process_zones(
        player: Query<&Transform, With<Player>>,
        mut zones: Query<&mut LevelZone>,
        mut level_tx: EventWriter<LevelGameplayEvent>,
    ) {
        let Ok(player_xform) = player.get_single() else { return; };

        for mut zone in zones.iter_mut() {
            let contains_player = zone.contains(player_xform);

            if zone.contains_player != contains_player {
                if contains_player {
                    log::trace!("Zone entered: {}", zone.name);

                    level_tx.send(LevelGameplayEvent::ZoneEntered(
                        zone.name.clone(),
                    ));
                } else {
                    log::trace!("Zone left: {}", zone.name);

                    level_tx
                        .send(LevelGameplayEvent::ZoneLeft(zone.name.clone()));
                }

                zone.contains_player = contains_player;
            }
        }
    }

    pub fn unload(
        mut commands: Commands,
        mut goto_level_rx: EventReader<GotoLevel>,
        entities: Query<
            Entity,
            Or<(
                With<AssetHandle<Model>>,
                With<Light>,
                With<LevelZone>,
                With<GcAfterLevelUnloaded>,
            )>,
        >,
    ) {
        if goto_level_rx.iter().count() == 0 {
            return;
        }

        if !entities.is_empty() {
            log::info!("Unloading previous level");
        }

        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Level(usize);

impl Level {
    pub fn l1() -> Self {
        Self(1)
    }

    pub fn l2() -> Self {
        Self(2)
    }
}

impl FromStr for Level {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self).map_err(|_| "Not a valid number")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GotoLevel(Level);

impl GotoLevel {
    pub fn new(level: Level) -> Self {
        Self(level)
    }
}

impl ops::Deref for GotoLevel {
    type Target = Level;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub enum LevelGameplayEvent {
    ZoneEntered(String),
    ZoneLeft(String),
}
