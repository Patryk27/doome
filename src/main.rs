use doome_engine::{Canvas, Color, HEIGHT, WIDTH};

struct App;

fn main() {
    doome_engine::main(App);
}

impl doome_engine::App for App {
    fn update(&mut self) {
        //
    }

    fn draw(&self, mut canvas: Canvas<'_>) {
        canvas.rect(
            0,
            0,
            WIDTH,
            HEIGHT,
            Color {
                r: 0x48,
                g: 0xb2,
                b: 0xe8,
                a: 0xff,
            },
        );
    }
}
