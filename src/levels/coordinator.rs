use std::ops;
use std::str::FromStr;

use crate::prelude::*;

pub struct LevelsCoordinator;

impl LevelsCoordinator {
    pub fn init(mut goto_level_tx: EventWriter<GotoLevel>) {
        goto_level_tx.send(GotoLevel::new(Level::l2()));
    }

    pub fn unload(
        mut commands: Commands,
        mut goto_level_rx: EventReader<GotoLevel>,
        entities: Query<
            Entity,
            Or<(
                With<AssetHandle<Model>>,
                With<Light>,
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
