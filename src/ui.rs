mod messages;
mod text;
mod typewriter;

// sub systems
mod angrey;
mod command_line;
mod gun;
mod health;

use bevy::prelude::*;
use doome_bevy::components::*;
use doome_bevy::doome::DoomeRenderer;
use doome_bevy::text::TextEngine;
use doome_engine::{Canvas, HEIGHT, WIDTH};
use glam::{vec2, Vec3Swizzles};

pub use self::messages::*;
pub use self::text::*;
pub use self::typewriter::*;

pub struct UiPlugin;

macro_rules! ordered_systems {
    ($name:path => $($args:tt)*) => {
        $name.pipe(ordered_systems!($($args)*))
    };

    ($name:path) => {
        $name
    };
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Typewriter
        app.insert_resource(Typewriter::Idle)
            .add_event::<TypewriterPrint>()
            .add_system(typewriter::update);

        // Messages
        app.insert_resource(Messages::default())
            .add_event::<Message>()
            .add_system(messages::update);

        // Setups
        app.add_startup_system(command_line::setup);
        app.add_startup_system(gun::setup);

        // Updates
        app.add_system(command_line::update);

        // TODO: Extract this stuff
        // Gun
        app.add_system(gun::trigger_shoot);
        app.add_system(gun::update_gun);

        // Ui rendering systems (strictly ordered)
        app.add_system(ordered_systems! {
            clear
            => gun::render
            => angrey::render
            => health::render
            => render_texts
            => command_line::render
        });
    }
}

#[derive(Resource)]
pub struct InputLock {
    pub is_locked: bool,
}

fn clear(mut renderer: ResMut<DoomeRenderer>, text_engine: Res<TextEngine>) {
    let frame = &mut renderer.pixels.image_data;
    let mut canvas = Canvas::new_text(&text_engine, frame);

    canvas.clear();
}

fn render_texts(
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
