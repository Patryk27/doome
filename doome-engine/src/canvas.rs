use doome_surface::Surface as _;
use doome_text::TextEngine;

use crate::*;

pub struct Canvas<'frame> {
    text_engine: &'frame TextEngine,
    frame: &'frame mut [u8],
}

impl<'f> Canvas<'f> {
    pub(super) fn new(
        text_engine: &'f TextEngine,
        frame: &'f mut [u8],
    ) -> Self {
        Self { text_engine, frame }
    }

    pub fn set(&mut self, x: u16, y: u16, color: Color) {
        Surface { frame: self.frame }.set(x, y, color);
    }

    pub fn rect(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, color: Color) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.set(x, y, color);
            }
        }
    }

    pub fn text(&mut self, x: u16, y: u16, text: impl AsRef<str>) {
        let mut surface = Surface { frame: self.frame };

        self.text_engine.draw(&mut surface, x, y, text.as_ref());
    }
}

struct Surface<'frame> {
    frame: &'frame mut [u8],
}

impl<'f> doome_surface::Surface<'f> for Surface<'f> {
    fn set(&mut self, x: u16, y: u16, mut color: Color) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let idx = 4 * ((y as usize) * (WIDTH as usize) + (x as usize));

        if color.a < 255 {
            color.r = blend(color.r, self.frame[idx], color.a);
            color.g = blend(color.g, self.frame[idx + 1], color.a);
            color.b = blend(color.b, self.frame[idx + 2], color.a);
        }

        self.frame[idx] = color.r;
        self.frame[idx + 1] = color.g;
        self.frame[idx + 2] = color.b;
        self.frame[idx + 3] = 255;
    }
}

fn blend(x: u8, y: u8, a: u8) -> u8 {
    let a = (a as f32) / 255.0;
    let x = (x as f32) / 255.0;
    let y = (y as f32) / 255.0;

    ((a * x + (1.0 - a) * y) * 255.0) as _
}
