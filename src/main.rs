use self::engine::{Canvas, Color, HEIGHT, WIDTH};

mod engine;
mod raytracer;

struct App;

fn main() {
    engine::main(App);
}

impl engine::App for App {
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
