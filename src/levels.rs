use bevy::prelude::EventWriter;

pub use self::coordinator::*;

pub mod level1;
pub mod level2;

mod coordinator;
mod utils;

pub fn start(mut enter_levels: EventWriter<EnterLevel>) {
    enter_levels.send(EnterLevel::l2());
}
