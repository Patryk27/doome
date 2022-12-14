use doome_surface::{Color, Surface};
use rusttype::{point, Font, Scale};

pub struct TextEngine {
    fonts: Fonts,
}

impl TextEngine {
    pub fn draw(
        &self,
        surface: &mut dyn Surface,
        x: i16,
        y: i16,
        text: &str,
        centered: bool,
    ) {
        let font = &self.fonts.minecraft;
        let scale = Scale::uniform(14.0);
        let offset = point(0.0, font.v_metrics(scale).ascent);

        let x = if centered {
            let width = font
                .layout(text, scale, offset)
                .flat_map(|glyph| glyph.pixel_bounding_box())
                .last()
                .map(|bb| bb.max.x as i16)
                .unwrap_or(0);

            x - width / 2
        } else {
            x
        };

        for glyph in font.layout(text, scale, offset) {
            let bb = if let Some(bb) = glyph.pixel_bounding_box() {
                bb
            } else {
                continue;
            };

            glyph.draw(|gx, gy, gv| {
                let gx = (x as i32) + (gx as i32 + bb.min.x);
                let gy = (y as i32) + (gy as i32 + bb.min.y);

                if let (Ok(gx), Ok(gy)) = (gx.try_into(), gy.try_into()) {
                    surface.set(
                        gx,
                        gy,
                        Color {
                            r: 255,
                            g: 255,
                            b: 255,
                            a: (gv * 255.0) as u8,
                        },
                    );
                }
            });
        }
    }
}

impl Default for TextEngine {
    fn default() -> Self {
        let minecraft = include_bytes!("../fonts/Minecraft.ttf");

        Self {
            fonts: Fonts {
                minecraft: Font::try_from_bytes(minecraft).unwrap(),
            },
        }
    }
}

struct Fonts {
    minecraft: Font<'static>,
}
