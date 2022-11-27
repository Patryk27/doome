use std::ops;
use std::str::FromStr;

use crate::prelude::*;

#[derive(Resource)]
pub struct LevelsCoordinator {
    pub current_level: Level,
    pub is_game_over: bool,
    pub time_in_game_over: f32,
}

impl LevelsCoordinator {
    pub fn init(
        mut commands: Commands,
        mut goto_level_tx: EventWriter<GotoLevel>,
    ) {
        let starting_level = Level::l5();

        let levels_coordinator = LevelsCoordinator {
            current_level: starting_level,
            is_game_over: false,
            time_in_game_over: 0.0,
        };

        commands.insert_resource(levels_coordinator);

        goto_level_tx.send(GotoLevel::new(starting_level));
    }

    pub fn handle_game_state(
        mut this: ResMut<LevelsCoordinator>,
        mut goto_level_tx: EventReader<GotoLevel>,
    ) {
        let last_level = goto_level_tx.iter().last().copied();

        if let Some(level) = last_level {
            if *level == Level::l0() {
                this.is_game_over = true;
                this.time_in_game_over = 0.0;
            } else {
                this.is_game_over = false;
                this.current_level = *level;
            }
        }
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
        mut inventory: Query<&mut Inventory>,
        mut change_hud_visibility_tx: EventWriter<ChangeHudVisibility>,
    ) {
        if goto_level_rx.iter().count() == 0 {
            return;
        }

        if !entities.is_empty() {
            log::info!("Unloading previous level");
        }

        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        if let Ok(mut inventory) = inventory.get_single_mut() {
            *inventory = Default::default();
        }

        change_hud_visibility_tx.send(ChangeHudVisibility::show());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Level(usize);

impl Level {
    pub fn l0() -> Self {
        Self(0)
    }

    pub fn l1() -> Self {
        Self(1)
    }

    pub fn l2() -> Self {
        Self(2)
    }

    pub fn l3() -> Self {
        Self(3)
    }

    pub fn l4() -> Self {
        Self(4)
    }

    pub fn l5() -> Self {
        Self(5)
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
    DoorOpened(String),
    KeyPicked(String),
    ZoneEntered(String),
    ZoneLeft(String),
}
