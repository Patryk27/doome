use bevy::prelude::*;
use doome_bevy::components::*;
use doome_bevy::events::SyncStaticGeometry;

use super::utils::*;

pub fn init(mut commands: Commands, mut tx: EventWriter<SyncStaticGeometry>) {
    commands.spawn((
        Player,
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Rotation { angle: 0.0 },
    ));

    commands.floor(-3, -3, 3, 3);
    commands.wall(-3, 3, -1, 3, 0);
    commands.wall(1, 3, 3, 3, 0);
    commands.wall(3, 3, 3, -3, 1);
    commands.wall(-3, -3, 3, -3, 2);
    commands.wall(-3, -3, -3, 3, 3);
    commands.floor(-1, 3, 1, 5);
    commands.wall(-1, 5, 1, 5, 0);
    commands.wall(1, 3, 1, 5, 1);
    commands.wall(-1, 3, -1, 5, 3);
    commands.ceiling(-10, -10, 10, 10);
    commands.light(0.0, 2.0, 0.0, 1.0, 1.0, 1.0);

    tx.send(SyncStaticGeometry);
}
