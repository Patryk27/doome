use doome_engine::Canvas;

pub struct Text<'a> {
    font: &'a Font,
    text: &'a str,
}

impl<'a> Text<'a> {
    pub fn new(font: &'a Font, text: &'a str) -> Self {
        Self { font, text }
    }

    pub fn draw(&self, canvas: &mut Canvas<'_>) {
        //
    }
}

pub enum Font {
    Example,
}
