use crate::*;

pub struct Canvas<'frame> {
    frame: &'frame mut [u8],
}

impl<'f> Canvas<'f> {
    pub(super) fn new(frame: &'f mut [u8]) -> Self {
        Self { frame }
    }

    pub fn set(&mut self, x: u16, y: u16, color: Color) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let idx = 4 * ((y * WIDTH + x) as usize);

        self.frame[idx] = color.r;
        self.frame[idx + 1] = color.g;
        self.frame[idx + 2] = color.b;
        self.frame[idx + 3] = color.a;
    }

    pub fn rect(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, color: Color) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.set(x, y, color);
            }
        }
    }
}
