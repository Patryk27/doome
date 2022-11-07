mod engine;

const BOX_SIZE: i16 = 64;

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
        if self.box_x <= 0 || self.box_x + BOX_SIZE > 320 {
            self.velocity_x *= -1;
        }

        if self.box_y <= 0 || self.box_y + BOX_SIZE > 200 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % 320 as usize) as i16;
            let y = (i / 200 as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
