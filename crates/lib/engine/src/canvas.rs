use doome_surface::{Color, Surface as _};
use doome_text::TextEngine;
use image::RgbaImage;

use crate::*;

pub type TextCanvas<'frame> = Canvas<'frame, TextEngine>;

pub struct Canvas<'frame, T = ()> {
    frame: &'frame mut [u8],
    text_engine: &'frame T,
}

impl<'f> Canvas<'f, ()> {
    pub fn new(frame: &'f mut [u8]) -> Self {
        Self {
            frame,
            text_engine: &(),
        }
    }
}

impl<'f, T> Canvas<'f, T> {
    pub fn clear(&mut self) {
        self.frame.fill(0);
    }

    pub fn fill(&mut self, color: Color) {
        self.frame.fill(color.a);
        self.frame.chunks_exact_mut(4).for_each(|chunk| {
            chunk[0] = color.r;
            chunk[1] = color.g;
            chunk[2] = color.b;
            chunk[3] = color.a;
        });
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

    pub fn blit_blended(
        &mut self,
        x_offset: u16,
        y_offset: u16,
        image: &RgbaImage,
        blend_color: Color,
    ) {
        let (width, height) = image.dimensions();

        for x in 0..width {
            for y in 0..height {
                let color = image.get_pixel(x, y).0;

                let color = Color {
                    r: color[0],
                    g: color[1],
                    b: color[2],
                    a: color[3],
                }
                .blend(blend_color);

                self.set(x_offset + x as u16, y_offset + y as u16, color);
            }
        }
    }

    pub fn blit(&mut self, x_offset: u16, y_offset: u16, image: &RgbaImage) {
        self.blit_blended(x_offset, y_offset, image, Color::WHITE)
    }
}

// Text specific
impl<'frame> Canvas<'frame, TextEngine> {
    pub fn new_text(
        text_engine: &'frame TextEngine,
        frame: &'frame mut [u8],
    ) -> Self {
        Self { frame, text_engine }
    }

    pub fn text(
        &mut self,
        x: u16,
        y: u16,
        text: impl AsRef<str>,
        centered: bool,
    ) {
        let mut surface = Surface { frame: self.frame };

        self.text_engine
            .draw(&mut surface, x, y, text.as_ref(), centered);
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

        let ptr = 4 * ((y as usize) * (WIDTH as usize) + (x as usize));

        if color.a < 255 {
            color.r = blend(color.r, self.frame[ptr], color.a);
            color.g = blend(color.g, self.frame[ptr + 1], color.a);
            color.b = blend(color.b, self.frame[ptr + 2], color.a);
            color.a = blend(color.a, self.frame[ptr + 3], color.a);
        }

        self.frame[ptr] = color.r;
        self.frame[ptr + 1] = color.g;
        self.frame[ptr + 2] = color.b;
        self.frame[ptr + 3] = color.a;
    }
}

fn blend(x: u8, y: u8, a: u8) -> u8 {
    let a = (a as f32) / 255.0;
    let x = (x as f32) / 255.0;
    let y = (y as f32) / 255.0;

    ((a * x + (1.0 - a) * y) * 255.0) as _
}
