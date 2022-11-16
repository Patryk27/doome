use bevy::prelude::{Plugin, Resource};
use engine::{HEIGHT, WIDTH};
use pixels::wgpu::{CommandEncoder, TextureView};
use pixels::{Pixels, PixelsContext, SurfaceTexture};

pub struct PixelsPlugin;

impl Plugin for PixelsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let windows = app.world.resource::<bevy::window::Windows>();
        let window = windows.get_primary().unwrap();
        let window_id = window.id();

        let winit_windows = app
            .world
            .non_send_resource_mut::<bevy::winit::WinitWindows>();

        let window = winit_windows.get_window(window_id).unwrap();
        let window_size = window.inner_size();

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &window);

        let pixels =
            Pixels::new(WIDTH as _, HEIGHT as _, surface_texture).unwrap();

        app.insert_resource(PixelsState {
            window_size: (window_size.width, window_size.height),
            pixels,
        });
    }
}

#[derive(Resource)]
pub struct PixelsState {
    window_size: (u32, u32),
    pixels: Pixels,
}

impl PixelsState {
    pub fn render(
        &mut self,
        window_size: (u32, u32),
        f: impl FnOnce(&mut CommandEncoder, &TextureView, &PixelsContext),
    ) {
        if window_size != self.window_size {
            log::info!("Resizing: {:?} -> {:?}", self.window_size, window_size);

            self.pixels.resize_surface(window_size.0, window_size.1);
            self.window_size = window_size;
        }

        self.pixels
            .render_with(|encoder, view, context| {
                f(encoder, view, context);
                Ok(())
            })
            .unwrap();
    }

    pub fn inner(&self) -> &Pixels {
        &self.pixels
    }

    pub fn inner_mut(&mut self) -> &mut Pixels {
        &mut self.pixels
    }
}
