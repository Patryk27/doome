use std::time::Duration;

use bevy::prelude::*;
use doome_engine::{TextCanvas, HEIGHT};

#[derive(Default, Resource)]
pub struct Messages {
    items: Vec<Message>,
}

impl Messages {
    pub fn render(&self, canvas: &mut TextCanvas) {
        if let Some(item) = self.items.get(0) {
            canvas.text(5, HEIGHT - 21, &item.text, false);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    text: String,
    timer: Timer,
}

impl Message {
    pub fn _new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
        }
    }
}

pub fn update(
    time: Res<Time>,
    mut state: ResMut<Messages>,
    mut new_messages: EventReader<Message>,
) {
    for new_message in new_messages.iter() {
        state.items.push(new_message.to_owned());
    }

    if let Some(item) = state.items.get_mut(0) {
        item.timer.tick(time.delta());

        if item.timer.just_finished() {
            state.items.remove(0);
        }
    }
}
