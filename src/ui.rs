mod angrey;
mod blood;
mod console;
mod game_over;
pub mod gun;
mod health;
mod menu;
mod messages;
mod text;
mod typewriter;

use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::components::*;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::prelude::DoomeAssets;
use doome_bevy::text::TextEngine;
use doome_engine::{Canvas, HEIGHT, WIDTH};
use glam::{vec2, Vec3Swizzles};

pub use self::messages::*;
pub use self::text::*;
pub use self::typewriter::*;
use crate::prelude::Inventory;
use crate::weapons::PrefabWeapons;

pub const HEALTHY_THRESHOLD: f32 = 50.0;
pub const WOUNDED_THRESHOLD: f32 = 25.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let prefab_weapons = app.world.resource::<PrefabWeapons>();

        app.insert_resource(gun::State::new(&prefab_weapons))
            .insert_resource(InputLock { is_locked: false })
            .insert_resource(UiState { hud_visible: false })
            .add_event::<ChangeHudVisibility>();

        // Miscellaneous
        #[cfg(target_arch = "wasm32")]
        app.add_system(grab_cursor);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_system(grab_cursor);

        // Typewriter
        app.insert_resource(Typewriter::default())
            .add_event::<TypewriterPrint>()
            .add_event::<TypewriterPrintingCompleted>()
            .add_system(typewriter::update);

        // Messages
        app.insert_resource(Messages::default())
            .add_event::<Message>()
            .add_system(messages::update);

        // Blood
        app.add_startup_system(blood::setup);

        // Game Over
        app.add_startup_system(game_over::setup);

        // Console
        app.add_startup_system(console::setup)
            .add_system(console::update);

        // Menu
        app.add_startup_system(menu::setup).add_system(menu::update);

        // Ui rendering systems (strictly ordered)
        app.add_system(ordered_systems! {
            process_events
            => canvas_clear
            => blood::render
            => gun::render
            => angrey::render
            => health::render
            => canvas_render_texts
            => game_over::render
            => console::render
            => menu::render
        });
    }
}

pub struct ChangeHudVisibility {
    pub visible: bool,
}

impl ChangeHudVisibility {
    pub fn show() -> Self {
        Self { visible: true }
    }

    pub fn hide() -> Self {
        Self { visible: false }
    }
}

#[derive(Resource)]
pub struct InputLock {
    pub is_locked: bool,
}

#[derive(Resource)]
pub struct UiState {
    pub hud_visible: bool,
}

#[cfg(target_arch = "wasm32")]
fn grab_cursor(
    mut windows: ResMut<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.pressed(MouseButton::Left)
        || mouse_button_input.pressed(MouseButton::Right)
    {
        let window = windows.primary_mut();

        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        window.set_cursor_visibility(false);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn grab_cursor(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();

    window.set_cursor_visibility(false);

    #[cfg(target_os = "macos")]
    window.set_cursor_grab_mode(CursorGrabMode::Locked);

    #[cfg(not(target_os = "macos"))]
    window.set_cursor_grab_mode(CursorGrabMode::Confined);
}

fn process_events(
    mut change_hud_visibility_rx: EventReader<ChangeHudVisibility>,
    mut ui: ResMut<UiState>,
) {
    for event in change_hud_visibility_rx.iter() {
        ui.hud_visible = event.visible;
    }
}

fn canvas_clear(
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new_text(&text_engine, frame);

    canvas.clear();
}

fn canvas_render_texts(
    assets: Res<DoomeAssets>,
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
    typewriter: Res<Typewriter>,
    messages: Res<Messages>,
    texts: Query<(&Text, &Transform, Option<&Visibility>)>,
    inventory: Query<&Inventory>,
) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new_text(&text_engine, frame);

    typewriter.render(&mut canvas);
    messages.render(&mut canvas);

    for (text, transform, visibility) in texts.iter() {
        let is_visible = visibility.map_or(true, |vis| vis.is_visible);

        if !is_visible {
            continue;
        }

        let translation =
            transform.translation.xy() * vec2(WIDTH as _, HEIGHT as _);

        canvas.text(
            translation.x as _,
            translation.y as _,
            &text.text,
            text.centered,
        );
    }

    if let Ok(inventory) = inventory.get_single() {
        let key_image = assets.load_image("key");
        let key_image = assets.image(key_image);

        for (key_idx, key) in inventory.keys.iter().enumerate() {
            let color = key.color();
            let color = doome_surface::Color {
                r: (color.r * 255.0) as u8,
                g: (color.g * 255.0) as u8,
                b: (color.b * 255.0) as u8,
                a: 255,
            };

            canvas.blit_blended(
                WIDTH - 16 - 5,
                5 + (key_idx as u16) * 16,
                key_image,
                color,
            );
        }
    }
}
