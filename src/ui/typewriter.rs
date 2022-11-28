const FUEL_PER_CHARACTER: f32 = 0.06;
const LINE_HEIGHT: u16 = 12;

use std::collections::VecDeque;
use std::mem;

use doome_engine::{TextCanvas, HEIGHT, WIDTH};
use doome_surface::Color;

use crate::prelude::*;

#[derive(Default, Resource)]
pub struct Typewriter {
    current: Option<TypewriterText>,
    scheduled: VecDeque<TypewriterText>,
}

impl Typewriter {
    pub fn render(&self, canvas: &mut TextCanvas) {
        if let Some(current) = &self.current {
            current.render(canvas);
        }
    }
}

#[derive(Default)]
pub struct TypewriterPrint {
    id: Option<String>,
    text: String,
}

impl TypewriterPrint {
    pub fn new(text: impl ToString) -> Self {
        Self {
            id: None,
            text: text.to_string(),
        }
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }

    fn build(&self) -> TypewriterText {
        TypewriterText {
            id: self.id.clone(),
            text: self.text.clone(),
            layout: Default::default(),
            tt: Default::default(),
            relayout_at: Default::default(),
            completed_at: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypewriterPrintingCompleted {
    pub id: String,
}

#[derive(Clone, Debug)]
struct TypewriterText {
    id: Option<String>,
    text: String,
    layout: Vec<String>,
    tt: f32,
    relayout_at: f32,
    completed_at: Option<f32>,
}

impl TypewriterText {
    fn render(&self, canvas: &mut TextCanvas) {
        let x = 5;
        let mut y = 5;

        canvas.rect(
            0,
            0,
            WIDTH,
            5 + (self.layout.len() as u16) * LINE_HEIGHT + 5,
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 128 + 64,
            },
        );

        for line in self.layout.iter() {
            canvas.text(x, y, line, false);

            y += LINE_HEIGHT;
        }
    }

    fn layout(text: &str, mut fuel: f32) -> (Vec<String>, f32) {
        let mut layout = Vec::new();

        // ----

        let mut caret_allowed = true;
        let mut text = text.chars().peekable();
        let mut layout_line = String::new();

        while let Some(ch) = text.next() {
            if ch == '%' && text.peek() == Some(&'{') {
                let _ = text.next();

                match text.next() {
                    Some('c') => {
                        assert_eq!(Some('}'), text.next());
                        layout.clear();
                    }

                    Some('h') => {
                        assert_eq!(Some('}'), text.next());
                        caret_allowed = false;
                    }

                    Some('s') => {
                        let sleep_time: f32 = (&mut text)
                            .take_while(|&ch| ch != '}')
                            .collect::<String>()
                            .parse()
                            .unwrap();

                        fuel -= sleep_time;
                    }

                    _ => {
                        panic!();
                    }
                }

                continue;
            }

            fuel -= if caret_allowed {
                Self::fuel(ch)
            } else {
                FUEL_PER_CHARACTER / 2.0
            };

            if fuel < 0.0 {
                if caret_allowed {
                    layout_line.push('_');
                }

                break;
            }

            if ch == '\n' {
                layout.push(mem::take(&mut layout_line));
            } else {
                layout_line.push(ch);
            }
        }

        if !layout_line.trim().is_empty() {
            layout.push(layout_line);
        }

        // ----

        loop {
            let required_height = LINE_HEIGHT * (layout.len() as u16);

            if required_height + LINE_HEIGHT > HEIGHT {
                layout.remove(0);
            } else {
                break;
            }
        }

        (layout, fuel)
    }

    fn fuel(ch: char) -> f32 {
        match ch {
            '\n' => 3.5 * FUEL_PER_CHARACTER,
            ' ' => 1.25 * FUEL_PER_CHARACTER,
            ch if (ch as u8) % 2 == 0 => 1.5 * FUEL_PER_CHARACTER,
            _ => FUEL_PER_CHARACTER,
        }
    }
}

pub fn update(
    time: Res<Time>,
    mut state: ResMut<Typewriter>,
    mut events: EventReader<TypewriterPrint>,
    ui: Res<UiState>,
    keys: Res<Input<KeyCode>>,
    mut goto_level_rx: EventReader<GotoLevel>,
    mut completed_tx: EventWriter<TypewriterPrintingCompleted>,
) {
    if goto_level_rx.iter().count() > 0 {
        *state = Default::default();
        return;
    }

    if let Some(curr) = &mut state.current {
        let speed = if ui.hud_visible {
            1.0
        } else {
            if keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::Return) {
                4.0
            } else {
                1.0
            }
        };

        curr.tt += speed * time.delta_seconds();

        if curr.tt >= curr.relayout_at {
            let (new_layout, remaining_fuel) =
                TypewriterText::layout(&curr.text, curr.tt);

            curr.layout = new_layout;

            if remaining_fuel > 0.0 && curr.completed_at.is_none() {
                curr.completed_at = Some(curr.tt);
            }

            curr.relayout_at = curr.tt + FUEL_PER_CHARACTER;
        }

        if let Some(completed_at) = curr.completed_at {
            if curr.tt > completed_at + 2.0 {
                if let Some(id) = &curr.id {
                    completed_tx.send(TypewriterPrintingCompleted {
                        id: id.to_owned(),
                    });
                }

                state.current = state.scheduled.pop_front();
            }
        }
    }

    for event in events.iter() {
        if state.current.is_none() {
            state.current = Some(event.build());
        } else {
            state.scheduled.push_back(event.build());
        }
    }
}
