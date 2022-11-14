use doome_engine::{Canvas, HEIGHT, HUD_HEIGHT, WIDTH};
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
                r: 0x10,
                g: 0x10,
                b: 0x10,
                a: 0xff,
            },
        );

        // canvas.text(
        //     10,
        //     HEIGHT - HUD_HEIGHT / 2 - 7,
        //     format!("Hello, World -- it's frame #{}!", self.frame),
        // );
    }
}
