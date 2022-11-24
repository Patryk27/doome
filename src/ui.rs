mod angrey;
mod command_line;
mod gun;
mod health;
mod messages;
mod text;
mod typewriter;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::components::*;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::text::TextEngine;
use doome_engine::{Canvas, HEIGHT, WIDTH};
use glam::{vec2, Vec3Swizzles};

pub use self::messages::*;
pub use self::text::*;
pub use self::typewriter::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputLock { is_locked: false });

        // Miscellaneous
        app.add_startup_system(hide_cursor).add_system(quit_on_exit);

        // Typewriter
        app.insert_resource(Typewriter::Idle)
            .add_event::<TypewriterPrint>()
            .add_system(typewriter::update);

        // Messages
        app.insert_resource(Messages::default())
            .add_event::<Message>()
            .add_system(messages::update);

        // Command line
        app.add_startup_system(command_line::setup)
            .add_system(command_line::update);

        // Gun animation
        // TODO: Extract this stuff
        app.add_startup_system(gun::setup)
            .add_system(gun::trigger_shoot)
            .add_system(gun::update_gun);

        // Ui rendering systems (strictly ordered)
        app.add_system(ordered_systems! {
            canvas_clear
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
    mut renderer: ResMut<DoomeRenderer>,
    text_engine: Res<TextEngine>,
    typewriter: Res<Typewriter>,
    messages: Res<Messages>,
    texts: Query<(&Text, &Transform, Option<&Visibility>)>,
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
}
