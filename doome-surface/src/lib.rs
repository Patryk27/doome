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
