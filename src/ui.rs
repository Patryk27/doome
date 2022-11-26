mod angrey;
mod blood;
mod command_line;
pub mod gun;
mod health;
mod messages;
mod text;
mod typewriter;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::components::*;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::prelude::Assets;
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

        app.insert_resource(gun::State::new(&prefab_weapons));
        app.insert_resource(InputLock { is_locked: false });

        // Miscellaneous
        app.add_startup_system(hide_cursor).add_system(quit_on_exit);

        // Typewriter
        app.insert_resource(Typewriter::default())
            .add_event::<TypewriterPrint>()
            .add_system(typewriter::update);

        // Messages
        app.insert_resource(Messages::default())
            .add_event::<Message>()
            .add_system(messages::update);

        // Command line
        app.add_startup_system(command_line::setup)
            .add_system(command_line::update);

        // Blood
        app.add_startup_system(blood::setup);

        // Ui rendering systems (strictly ordered)
        app.add_system(ordered_systems! {
            canvas_clear
            => blood::render
            => gun::render
            => angrey::render
            => health::render
            => canvas_render_texts
            => command_line::render
        });
    }
}

#[derive(Resource)]
pub struct InputLock {
    pub is_locked: bool,
}

fn hide_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_grab_mode(CursorGrabMode::Confined);
    window.set_cursor_visibility(false);
}

fn quit_on_exit(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
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
    assets: Res<Assets>,
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
