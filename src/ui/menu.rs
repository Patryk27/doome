use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::text::TextEngine;
use doome_engine::{TextCanvas, HEIGHT, WIDTH};
use doome_surface::Color;

use crate::prelude::*;

const ITEMS: &[&[&str]] = &[
    &["Continue game", "Adjust mouse sensitivity", "Quit game"],
    &["Increase sensitivity", "Decrease sensitivity", "Confirm"],
];

#[derive(Default, Resource)]
pub struct Menu {
    is_visible: bool,
    menu_idx: usize,
    item_idx: usize,
}

impl Menu {
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(Menu::default());
}

pub fn update(
    keys: Res<Input<KeyCode>>,
    mut mouse_wheel_rx: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut settings: ResMut<Settings>,
    mut exit: EventWriter<AppExit>,
    mut state: ResMut<Menu>,
    mut lock: ResMut<InputLock>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        state.is_visible = !state.is_visible;
        lock.is_locked = state.is_visible;

        if state.is_visible {
            state.menu_idx = 0;
            state.item_idx = 0;
        }
    }

    if !state.is_visible {
        return;
    }

    // -----

    enum Action {
        MoveUp,
        MoveDown,
        Confirm,
    }

    let mut action = None;

    if keys.just_pressed(KeyCode::Up) || keys.just_pressed(KeyCode::W) {
        action = Some(Action::MoveUp);
    }

    if keys.just_pressed(KeyCode::Down) || keys.just_pressed(KeyCode::S) {
        action = Some(Action::MoveDown);
    }

    if keys.just_pressed(KeyCode::Return) {
        action = Some(Action::Confirm);
    }

    let wheel_y = mouse_wheel_rx.iter().map(|event| event.y).sum::<f32>();

    if wheel_y < 0.0 {
        action = Some(Action::MoveDown);
    } else if wheel_y > 0.0 {
        action = Some(Action::MoveUp);
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        action = Some(Action::Confirm);
    }

    // -----

    match action {
        Some(Action::MoveUp) => {
            state.item_idx = if state.item_idx == 0 {
                ITEMS[state.menu_idx].len() - 1
            } else {
                state.item_idx - 1
            };
        }

        Some(Action::MoveDown) => {
            state.item_idx = (state.item_idx + 1) % ITEMS[state.menu_idx].len();
        }

        Some(Action::Confirm) => match (state.menu_idx, state.item_idx) {
            (0, 0) => {
                state.is_visible = false;
                lock.is_locked = state.is_visible;
            }

            (0, 1) => {
                state.menu_idx = 1;
                state.item_idx = 0;
            }

            (0, 2) => {
                exit.send(AppExit);
            }

            (1, 0) => {
                settings.mouse_sensitivity += 0.05;
            }

            (1, 1) => {
                settings.mouse_sensitivity =
                    (settings.mouse_sensitivity - 0.05).max(0.05);
            }

            (1, 2) => {
                state.menu_idx = 0;
                state.item_idx = 0;
            }

            _ => unreachable!(),
        },

        None => {
            //
        }
    }
}

pub fn render(
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
    settings: Res<Settings>,
    state: Res<Menu>,
) {
    if !state.is_visible {
        return;
    }

    let frame = &mut renderer.pixels.image_data;
    let mut canvas = TextCanvas::new_text(&text_engine, frame);

    canvas.rect(0, 0, WIDTH, HEIGHT, Color::hex(0x000000ee));

    for (item_idx, item) in ITEMS[state.menu_idx].into_iter().enumerate() {
        let option = if item_idx == state.item_idx {
            format!("> {} <", item)
        } else {
            item.to_string()
        };

        canvas.text(
            WIDTH / 2,
            HEIGHT / 2 - 20 + 20 * (item_idx as u16),
            option,
            true,
        );
    }

    if state.menu_idx == 1 {
        canvas.text(
            WIDTH / 2,
            HEIGHT / 2 - 45,
            format!("Mouse sensitivity: {:.2}", settings.mouse_sensitivity),
            true,
        );
    }
}
