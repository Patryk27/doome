use std::fmt;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use bevy::prelude::Entity;
use glam::Vec3;

use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    // quit
    Quit,

    // lock-input
    LockInput,

    // unlock-input
    UnlockInput,

    // list-entities
    ListEntities,

    // Displays the position of a given entity
    //  Example: position player
    //  Example: pos player
    Position {
        entity: EntityOrPlayer,
    },

    // Example: move player 0,0,0
    Move {
        entity: EntityOrPlayer,
        position: Vec3,
    },

    // Example: set-health player 100
    SetHealth {
        entity: EntityOrPlayer,
        health: f32,
    },

    // Example: heal player 20
    Heal {
        entity: EntityOrPlayer,
        amount: f32,
    },

    // Spawns a moth monster at a given position
    // Example: spawn moth-monster 0,0,0
    Spawn {
        spawnable: Spawnable,
        position: Vec3,
    },

    Despawn {
        entity: EntityHandle,
    },

    Kill {
        entity: EntityHandle,
    },

    SyncNavData,
    NoClip,
    DumpPhysics,

    // Example: goto-level 2
    GotoLevel {
        level: Level,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Spawnable {
    MothMonster,
    Heart,
}

#[derive(Debug, Clone, Copy)]
pub enum EntityOrPlayer {
    Entity(Entity),
    Player,
}

#[derive(Debug, Clone, Copy)]
pub struct EntityHandle(pub Entity);

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let cmd = parts.next().context("No command")?;

        match cmd {
            "quit" => Ok(Command::Quit),
            "lock-input" => Ok(Command::LockInput),
            "unlock-input" => Ok(Command::UnlockInput),
            "list-entities" => Ok(Command::ListEntities),

            "pos" | "position" => {
                let entity = parts.next().context("No entity")?;
                let entity = entity.parse().context("Invalid entity")?;

                Ok(Command::Position { entity })
            }

            "move" => {
                let entity = parts.next().context("Missing entity")?;
                let entity = entity.parse()?;

                let position = parts.next().context("Missing position")?;
                let position = parse_vec3(position)?;

                Ok(Command::Move { entity, position })
            }

            "set-health" => {
                let entity = parts.next().context("Missing entity")?;
                let entity = entity.parse()?;

                let health = parts.next().context("Missing health")?;
                let health = health.parse()?;

                Ok(Command::SetHealth { entity, health })
            }

            "heal" => {
                let entity = parts.next().context("Missing entity")?;
                let entity = entity.parse()?;

                let amount = parts.next().context("Missing amount")?;
                let amount = amount.parse()?;

                Ok(Command::Heal { entity, amount })
            }

            "spawn" => {
                let spawnable = parts.next().context("Missing spawnable")?;
                let spawnable = spawnable.parse()?;

                let position = parts.next().context("Missing position")?;
                let position = parse_vec3(position)?;

                Ok(Command::Spawn {
                    spawnable,
                    position,
                })
            }

            "despawn" => {
                let entity = parts.next().context("Missing entity")?.parse()?;

                Ok(Command::Despawn { entity })
            }

            "kill" => {
                let entity = parts.next().context("Missing entity")?.parse()?;

                Ok(Command::Kill { entity })
            }

            "sync-nav-data" => Ok(Command::SyncNavData),
            "noclip" => Ok(Command::NoClip),
            "dump-physics" => Ok(Command::DumpPhysics),

            "goto-level" => {
                let level = parts
                    .next()
                    .context("Missing level-id")?
                    .parse()
                    .map_err(|err| anyhow!("{}", err))
                    .context("Invalid level-id")?;

                Ok(Command::GotoLevel { level })
            }

            _ => Err(anyhow!("Failed to parse command: {s}")),
        }
    }
}

fn parse_vec3(s: &str) -> Result<Vec3, anyhow::Error> {
    let mut parts = s.split(',');

    let x = parts.next().context("Missing x")?.parse()?;
    let y = parts.next().context("Missing y")?.parse()?;
    let z = parts.next().context("Missing z")?.parse()?;

    Ok(Vec3::new(x, y, z))
}

impl FromStr for EntityOrPlayer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "player" => Ok(EntityOrPlayer::Player),
            _ => {
                let entity: u64 = s.parse().context("Invalid entity")?;
                Ok(EntityOrPlayer::Entity(Entity::from_bits(entity)))
            }
        }
    }
}

impl FromStr for Spawnable {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "moth-monster" => Ok(Spawnable::MothMonster),
            "heart" => Ok(Spawnable::Heart),
            _ => Err(anyhow!("Invalid spawnable: {s}")),
        }
    }
}

impl FromStr for EntityHandle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, 'v');

        let index: u64 = parts.next().context("Missing index")?.parse()?;
        let generation: u64 =
            parts.next().context("Missing generation")?.parse()?;

        let entity = Entity::from_bits(index | (generation << 32));

        Ok(Self(entity))
    }
}

impl fmt::Display for EntityHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = self.0.to_bits();

        let index = bits & 0xFFFF_FFFF;
        let generation = bits >> 32;

        write!(f, "{}v{}", index, generation)
    }
}
