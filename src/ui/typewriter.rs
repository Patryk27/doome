const FUEL_PER_CHARACTER: f32 = 0.07;
const LINE_HEIGHT: u16 = 12;

use std::mem;

use bevy::prelude::*;
use doome_engine::{Canvas, HEIGHT};

#[derive(Resource)]
pub enum Typewriter {
    Idle,

    Working {
        text: String,
        layout: Vec<String>,
        tt: f32,
        next_layout_at: f32,
        completed_at: Option<f32>,
    },
}

impl Typewriter {
    pub fn render(&self, canvas: &mut Canvas) {
        let Self::Working { layout, ..} = self else { return; };

        let x = 5;
        let mut y = 5;

        for line in layout.iter() {
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

#[derive(Default)]
pub struct TypewriterPrint {
    text: String,
}

impl TypewriterPrint {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

pub fn update(
    time: Res<Time>,
    mut state: ResMut<Typewriter>,
    mut print: EventReader<TypewriterPrint>,
) {
    if let Typewriter::Working {
        text,
        layout,
        tt,
        next_layout_at,
        completed_at,
    } = &mut *state
    {
        *tt += time.delta_seconds();

        if *tt >= *next_layout_at {
            let (new_layout, remaining_fuel) = Typewriter::layout(text, *tt);

            *layout = new_layout;

            if remaining_fuel > 0.0 && completed_at.is_none() {
                *completed_at = Some(*tt);
            }

            *next_layout_at = *tt + FUEL_PER_CHARACTER;
        }

        if let Some(completed_at) = completed_at {
            if *tt > *completed_at + 2.0 {
                *state = Typewriter::Idle;
            }
        }
    }

    for print in print.iter() {
        *state = Typewriter::Working {
            text: print.text.clone(),
            layout: Default::default(),
            tt: Default::default(),
            next_layout_at: Default::default(),
            completed_at: Default::default(),
        };
    }
}
