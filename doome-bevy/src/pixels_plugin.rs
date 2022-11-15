use bevy::prelude::{Plugin, Resource};
use doome_engine::{HEIGHT, WIDTH};
use pixels::{Pixels, SurfaceTexture};

pub struct PixelsPlugin;

#[derive(Resource)]
pub struct PixelsState {
    pub pixels: Pixels,
}

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

        app.insert_resource(PixelsState { pixels });
    }
}
