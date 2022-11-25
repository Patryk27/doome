mod geometrized;
mod indexed;
mod locator;
mod map;
mod tileset;

use std::collections::{BTreeSet, HashMap};
use std::{cmp, fmt};

use serde::Deserialize;

pub use self::locator::LevelLocator;
use super::builder::LevelBuilder;
use crate::prelude::*;

pub struct LevelLoader {
    tmj_data: &'static str,
}

impl LevelLoader {
    pub fn load(tmj_data: &'static str) -> Self {
        Self { tmj_data }
    }

    pub fn spawn(self, lvl: &mut LevelBuilder) -> LevelLocator {
        log::info!("Loading tileset");

        let tileset = tileset::Tileset::from_tsx(include_str!(
            "../../assets/levels/tileset.tsx"
        ));

        log::info!("Loading map");
        let map = map::Map::from_tmj(self.tmj_data);

        log::debug!("Indexing map");
        let (imap, mut locator) = map.index(&tileset);

        log::debug!("Geometrizing map");
        let gmap = imap.geometrize();

        log::debug!("Spawning map");
        gmap.spawn(lvl);
        locator.spawn(lvl);

        log::debug!("Completed");

        locator
    }
}
