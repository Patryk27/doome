use doome_engine::{Canvas, HEIGHT, WIDTH};
use doome_surface::Color;

struct App {
    frame: usize,
}

fn main() {
    doome_engine::main(App { frame: 0 });
}

impl doome_engine::App for App {
    fn update(&mut self) {
        self.frame += 1;
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

        canvas.text(
            10,
            HEIGHT - 27,
            format!("Hello, World -- it's frame #{}!", self.frame),
        );
    }
}
