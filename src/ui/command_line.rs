use bevy::prelude::*;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::text::TextEngine;
use doome_engine::{TextCanvas, HEIGHT, WIDTH};
use doome_surface::Color;

use crate::commands::{Command, CommandOutput};

#[derive(Resource)]
pub struct CommandLine {
    is_shown: bool,
    current: String,
    buffer: Vec<String>,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(CommandLine {
        is_shown: false,
        current: String::new(),
        buffer: vec![],
    });
}

pub fn update(
    mut cmd_line: ResMut<CommandLine>,
    mut commands: EventWriter<Command>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut outputs: EventReader<CommandOutput>,
    keys: Res<Input<KeyCode>>,
) {
    let cmd_line = cmd_line.as_mut();

    for ev in char_evr.iter() {
        if cmd_line.is_shown {
            cmd_line.current.push(ev.char);
        }
    }

    for ev in outputs.iter() {
        cmd_line.buffer.push(ev.0.clone());
    }

    if keys.just_pressed(KeyCode::Grave) {
        cmd_line.is_shown = !cmd_line.is_shown;

        if cmd_line.is_shown {
            cmd_line.current.clear();
            commands.send(Command::LockInput);
        } else {
            commands.send(Command::UnlockInput);
        }
    }

    if !cmd_line.is_shown {
        return;
    }

    if keys.just_pressed(KeyCode::Back) {
        cmd_line.current.pop();
        cmd_line.current.pop();
    }

    if keys.just_pressed(KeyCode::Return) {
        match cmd_line.current.parse::<Command>() {
            Ok(cmd) => {
                cmd_line.buffer.push(cmd_line.current.clone());
                cmd_line.current.clear();
                commands.send(cmd);
            }
            Err(e) => {
                log::error!("Invalid command: {e:?}");
                cmd_line.buffer.push(format!("Error: {}", e));
                cmd_line.current.clear();
            }
        }
    }
}

pub fn render(
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
    cmd_line: Res<CommandLine>,
) {
    if !cmd_line.is_shown {
        return;
    }

    let frame = &mut renderer.pixels.image_data;
    let mut canvas = TextCanvas::new_text(&text_engine, frame);

    canvas.rect(0, 0, WIDTH, HEIGHT, Color::hex(0x00000066));
    canvas.text(0, HEIGHT - 15, &cmd_line.current, false);

    for (i, line) in cmd_line.buffer.iter().rev().enumerate() {
        if i > 10 {
            break;
        }

        canvas.text(0, HEIGHT - 15 * (i + 2) as u16, line, false);
    }
}
