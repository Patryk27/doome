use doome_bevy::doome::DoomeRenderer;
use doome_bevy::text::TextEngine;
use doome_engine::{TextCanvas, HEIGHT, WIDTH};
use doome_surface::Color;

use super::menu::Menu;
use crate::prelude::*;

#[derive(Default, Resource)]
pub struct Console {
    is_visible: bool,
    current: String,
    buffer: Vec<Line>,
}

struct Line {
    text: String,
    was_input: bool,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(Console::default());
}

pub fn update(
    mut state: ResMut<Console>,
    menu: Res<Menu>,
    mut commands: EventWriter<Command>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut outputs: EventReader<CommandOutput>,
    keys: Res<Input<KeyCode>>,
) {
    if menu.is_visible() {
        return;
    }

    let state = state.as_mut();

    for ev in char_evr.iter() {
        if state.is_visible {
            state.current.push(ev.char);
        }
    }

    for ev in outputs.iter() {
        state.buffer.push(Line {
            text: ev.0.clone(),
            was_input: false,
        });
    }

    if keys.just_pressed(KeyCode::Grave) {
        state.is_visible = !state.is_visible;

        if state.is_visible {
            state.current.clear();
            commands.send(Command::LockInput);
        } else {
            commands.send(Command::UnlockInput);
        }
    }

    if keys.just_pressed(KeyCode::Up) {
        if let Some(cmd) = state.buffer.iter().rev().find(|x| x.was_input) {
            state.current = cmd.text.clone();
        }
    }

    if !state.is_visible {
        return;
    }

    if keys.just_pressed(KeyCode::Back) {
        state.current.pop();
        state.current.pop();
    }

    if keys.just_pressed(KeyCode::Return) {
        match state.current.trim().parse::<Command>() {
            Ok(cmd) => {
                state.buffer.push(Line {
                    text: state.current.trim().to_owned(),
                    was_input: true,
                });

                state.is_visible = false;
                state.current.clear();
                commands.send(cmd);
                commands.send(Command::UnlockInput);
            }

            Err(e) => {
                log::error!("Invalid command: {e:?}");

                state.buffer.push(Line {
                    text: format!("Error: {}", e),
                    was_input: false,
                });

                state.current.clear();
            }
        }
    }
}

pub fn render(
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
    state: Res<Console>,
    menu: Res<Menu>,
) {
    if menu.is_visible() || !state.is_visible {
        return;
    }

    let frame = &mut renderer.pixels.image_data;
    let mut canvas = TextCanvas::new_text(&text_engine, frame);

    canvas.rect(0, 0, WIDTH, HEIGHT, Color::hex(0x00000066));
    canvas.text(5, HEIGHT - 21, format!("$ {}", state.current), false);

    for (i, line) in state.buffer.iter().rev().enumerate() {
        if i > 10 {
            break;
        }

        canvas.text(5, HEIGHT - 28 - 12 * (i + 1) as u16, &line.text, false);
    }
}
