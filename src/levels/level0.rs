use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::prelude::*;

const MIN_TIME_IN_GAME_OVER: f32 = 1.0;

pub fn init(
    mut commands: Commands,
    mut game_commands: EventWriter<Command>,
    assets: Res<Assets>,
    mut goto_level_rx: EventReader<GotoLevel>,
) {
    if !goto_level_rx.iter().any(|level| **level == Level::l0()) {
        return;
    }

    let mut lvl = LevelBuilder::new(&mut commands, &assets);

    lvl.floor(-1, -1, 1, 20)
        .alter_material(|mat| {
            mat.with_reflectivity(0.1)
                .with_reflection_color(Color::hex(0xffffff))
        })
        .spawn();

    game_commands.send(Command::LockInput);
}

pub fn process(
    time: Res<Time>,
    mut coordinator: ResMut<LevelsCoordinator>,
    mut game_commands: EventWriter<Command>,
    mut goto_level_rx: EventReader<GotoLevel>,
    mut keyboard_rx: EventReader<KeyboardInput>,
    mut mouse_button_rx: EventReader<MouseButtonInput>,
) {
    if goto_level_rx.iter().any(|level| **level != Level::l0()) {
        // Unlock input for the next level
        game_commands.send(Command::UnlockInput);
    }

    let num_keyboard_events = keyboard_rx.iter().count();
    let num_mouse_button_events = mouse_button_rx.iter().count();

    if !coordinator.is_game_over {
        return;
    }

    coordinator.time_in_game_over += time.delta_seconds();

    if coordinator.time_in_game_over > MIN_TIME_IN_GAME_OVER {
        let total = num_keyboard_events + num_mouse_button_events;

        if total != 0 {
            game_commands.send(Command::GotoLevel {
                level: coordinator.current_level,
            })
        }
    }
}
