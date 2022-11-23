use std::str::FromStr;

use anyhow::{anyhow, Context};
use bevy::prelude::Entity;
use glam::Vec3;

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
}

#[derive(Debug, Clone, Copy)]
pub enum EntityOrPlayer {
    Entity(Entity),
    Player,
}

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
