use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::rendering_options::RenderingOptions;
use doome_bevy::text::TextEngine;
use doome_engine::{TextCanvas, HEIGHT, WIDTH};
use doome_surface::Color;

use crate::prelude::*;

#[derive(Default, Resource)]
pub struct Menu {
    items: Vec<Vec<MenuItem>>,
    is_visible: bool,
    menu_idx: usize,
    item_idx: usize,
}

impl Menu {
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn add(&mut self, f: impl FnOnce(&mut MenuBuilder)) {
        let mut menu = MenuBuilder::default();

        f(&mut menu);

        self.items.push(menu.items);
    }
}

#[derive(Default)]
struct MenuBuilder {
    items: Vec<MenuItem>,
}

impl MenuBuilder {
    fn add(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    fn add_if(&mut self, cond: bool, item: MenuItem) {
        if cond {
            self.add(item);
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    MainContinueGame,
    MainRestartCurrentLevel,
    MainMouseSettings,
    MainDisplaySettings,
    MainQuitGame,
    //
    MouseIncreaseSensitivity,
    MouseDecreaseSensitivity,
    MouseConfirm,
    //
    DisplayToggleSSE,
    DisplayToggleMode,
    DisplayConfirm,
}

impl MenuItem {
    fn label(&self, windows: &Windows, sse_enabled: bool) -> &'static str {
        match self {
            MenuItem::MainContinueGame => "Continue game",
            MenuItem::MainRestartCurrentLevel => "Restart current level",
            MenuItem::MainMouseSettings => "Mouse settings",
            MenuItem::MainDisplaySettings => "Display settings",
            MenuItem::MainQuitGame => "Quit game",
            //
            MenuItem::MouseIncreaseSensitivity => "Increase sensitivity",
            MenuItem::MouseDecreaseSensitivity => "Decrease sensitivity",
            MenuItem::MouseConfirm => "Confirm",
            //
            MenuItem::DisplayToggleMode => {
                if windows.get_primary().unwrap().mode()
                    == WindowMode::Fullscreen
                {
                    "Make windowed"
                } else {
                    "Make fullscreen"
                }
            }
            MenuItem::DisplayToggleSSE => {
                if sse_enabled {
                    "Disable CRT-screen effect"
                } else {
                    "Enable CRT-screen effect"
                }
            }
            MenuItem::DisplayConfirm => "Confirm",
        }
    }

    fn exec(
        &self,
        windows: &mut Windows,
        exit: &mut EventWriter<AppExit>,
        settings: &mut Settings,
        state: &mut Menu,
        lock: &mut InputLock,
        rendering_options: &mut RenderingOptions,
        game_commands: &mut EventWriter<Command>,
        levels_coordinator: &LevelsCoordinator,
    ) {
        match self {
            MenuItem::MainContinueGame => {
                state.is_visible = false;
                lock.is_locked = state.is_visible;
            }

            MenuItem::MainRestartCurrentLevel => {
                game_commands.send(Command::GotoLevel {
                    level: levels_coordinator.current_level,
                });

                state.is_visible = false;
                lock.is_locked = state.is_visible;
            }

            MenuItem::MainMouseSettings => {
                state.menu_idx = 1;
                state.item_idx = 0;
            }

            MenuItem::MainDisplaySettings => {
                state.menu_idx = 2;
                state.item_idx = 0;
            }

            MenuItem::MainQuitGame => {
                exit.send(AppExit);
            }

            MenuItem::MouseIncreaseSensitivity => {
                settings.mouse_sensitivity += 0.05;
            }

            MenuItem::MouseDecreaseSensitivity => {
                settings.mouse_sensitivity =
                    (settings.mouse_sensitivity - 0.05).max(0.05);
            }

            MenuItem::MouseConfirm => {
                state.menu_idx = 0;
                state.item_idx = 0;
            }

            MenuItem::DisplayToggleSSE => {
                rendering_options.sse_enabled = !rendering_options.sse_enabled;
            }

            MenuItem::DisplayToggleMode => {
                let window = windows.get_primary_mut().unwrap();

                if window.mode() == WindowMode::Fullscreen {
                    window.set_mode(WindowMode::Windowed);
                } else {
                    window.set_mode(WindowMode::Fullscreen);
                }
            }

            MenuItem::DisplayConfirm => {
                state.menu_idx = 0;
                state.item_idx = 0;
            }
        }
    }
}

pub fn setup(mut commands: Commands) {
    let mut menu = Menu::default();

    #[cfg(target_arch = "wasm32")]
    let is_wasm = true;

    #[cfg(not(target_arch = "wasm32"))]
    let is_wasm = false;

    menu.add(|menu| {
        menu.add(MenuItem::MainContinueGame);
        menu.add(MenuItem::MainRestartCurrentLevel);
        menu.add(MenuItem::MainMouseSettings);
        menu.add_if(!is_wasm, MenuItem::MainDisplaySettings);
        menu.add_if(!is_wasm, MenuItem::MainQuitGame);
    });

    menu.add(|menu| {
        menu.add(MenuItem::MouseIncreaseSensitivity);
        menu.add(MenuItem::MouseDecreaseSensitivity);
        menu.add(MenuItem::MouseConfirm);
    });

    menu.add(|menu| {
        menu.add(MenuItem::DisplayToggleSSE);
        menu.add(MenuItem::DisplayToggleMode);
        menu.add(MenuItem::DisplayConfirm);
    });

    commands.insert_resource(menu);
}

pub fn update(
    mut windows: ResMut<Windows>,
    keys: Res<Input<KeyCode>>,
    mut mouse_wheel_rx: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut exit: EventWriter<AppExit>,
    mut settings: ResMut<Settings>,
    mut state: ResMut<Menu>,
    mut lock: ResMut<InputLock>,
    mut game_commands: EventWriter<Command>,
    mut rendering_options: ResMut<RenderingOptions>,
    levels_coordinator: Res<LevelsCoordinator>,
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
                state.items[state.menu_idx].len() - 1
            } else {
                state.item_idx - 1
            };
        }

        Some(Action::MoveDown) => {
            state.item_idx =
                (state.item_idx + 1) % state.items[state.menu_idx].len();
        }

        Some(Action::Confirm) => {
            let item = state.items[state.menu_idx][state.item_idx];

            item.exec(
                &mut windows,
                &mut exit,
                &mut settings,
                &mut state,
                &mut lock,
                &mut rendering_options,
                &mut game_commands,
                &levels_coordinator,
            );
        }

        None => {
            //
        }
    }
}

pub fn render(
    windows: Res<Windows>,
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
    rendering_options: Res<RenderingOptions>,
    settings: Res<Settings>,
    state: Res<Menu>,
) {
    if !state.is_visible {
        return;
    }

    let frame = &mut renderer.pixels.image_data;
    let mut canvas = TextCanvas::new_text(&text_engine, frame);

    canvas.rect(0, 0, WIDTH, HEIGHT, Color::hex(0x000000ee));

    let menu_height = (state.items[state.menu_idx].len() * 20) as i16;

    for (item_idx, item) in state.items[state.menu_idx].iter().enumerate() {
        let item = item.label(&windows, rendering_options.sse_enabled);

        let option = if item_idx == state.item_idx {
            format!("> {} <", item)
        } else {
            item.to_owned()
        };

        canvas.text(
            (WIDTH / 2) as i16,
            (HEIGHT / 2) as i16 - menu_height / 2 + 20 * (item_idx as i16),
            option,
            true,
        );
    }

    if state.menu_idx == 1 {
        canvas.text(
            (WIDTH / 2) as i16,
            (HEIGHT / 2) as i16 - menu_height / 2 - 30,
            format!("Mouse sensitivity: {:.2}", settings.mouse_sensitivity),
            true,
        );
    }
}
