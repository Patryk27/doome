use self::engine::{Canvas, Color, HEIGHT, WIDTH};

mod engine;

const BOX_SIZE: u16 = 64;

struct App {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() {
    engine::main(App {
        box_x: 24,
        box_y: 16,
        velocity_x: 1,
        velocity_y: 1,
    });
}

impl engine::App for App {
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + (BOX_SIZE as i16) > 320 {
            self.velocity_x *= -1;
        }

        if self.box_y <= 0 || self.box_y + (BOX_SIZE as i16) > 200 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
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

        canvas.rect(
            self.box_x as u16,
            self.box_y as u16,
            (self.box_x as u16) + BOX_SIZE,
            (self.box_y as u16) + BOX_SIZE,
            Color {
                r: 0x5e,
                g: 0x48,
                b: 0xe8,
                a: 0xff,
            },
        );
    }
}
