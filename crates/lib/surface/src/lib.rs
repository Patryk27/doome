pub trait Surface<'frame> {
    fn set(&mut self, x: u16, y: u16, color: Color);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn hex(hex: u32) -> Self {
        let [r, g, b, a] = hex.to_be_bytes();

        Self { r, g, b, a }
    }

    pub fn blend_ignore_alpha(mut self, r: f32, g: f32, b: f32) -> Self {
        self.r = ((self.r as f32) * r) as u8;
        self.g = ((self.g as f32) * g) as u8;
        self.b = ((self.b as f32) * b) as u8;
        self
    }

    pub fn blend(mut self, other: Self) -> Self {
        self.r = ((self.r as f32) * (other.r as f32) / 255.0) as u8;
        self.g = ((self.g as f32) * (other.g as f32) / 255.0) as u8;
        self.b = ((self.b as f32) * (other.b as f32) / 255.0) as u8;
        self.a = ((self.a as f32) * (other.a as f32) / 255.0) as u8;
        self
    }

    pub fn with_a(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
}

impl Color {
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };

    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
}
