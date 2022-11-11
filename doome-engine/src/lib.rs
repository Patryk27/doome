mod canvas;
mod scaling_texture_renderer;

use std::rc::Rc;

use doome_raytracer::Raytracer;
use doome_raytracer_shader_common as sc;
use doome_surface::Color;
use doome_text::TextEngine;
use glam::{vec2, vec3, vec4};
use instant::Instant;
use pixels::{Pixels, SurfaceTexture};
use sc::camera::Camera;
use sc::object::Object;
use sc::world::World;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub use self::canvas::*;
use self::scaling_texture_renderer::*;

pub const WIDTH: u16 = 320;
pub const RAYTRACER_HEIGHT: u16 = 200;
pub const HUD_HEIGHT: u16 = 50;
pub const HEIGHT: u16 = RAYTRACER_HEIGHT + HUD_HEIGHT;

const FPS: f32 = 60.0;
const SPF: f32 = 1.0 / FPS;

pub trait App {
    fn update(&mut self);
    fn draw(&self, canvas: Canvas<'_>);
}

pub fn main(app: impl App + 'static) {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Error).unwrap();
        wasm_bindgen_futures::spawn_local(run(app));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        dotenv::dotenv().ok();
        env_logger::init();
        pollster::block_on(run(app));
    }
}

async fn run(mut app: impl App + 'static) {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH * 4, HEIGHT * 4);

        WindowBuilder::new()
            .with_title("Doomé")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window = Rc::new(window);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        let get_window_size = || {
            let client_window = web_sys::window().unwrap();

            LogicalSize::new(
                client_window.inner_width().unwrap().as_f64().unwrap(),
                client_window.inner_height().unwrap().as_f64().unwrap(),
            )
        };

        let window = Rc::clone(&window);

        window.set_inner_size(get_window_size());

        let client_window = web_sys::window().unwrap();

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .unwrap();

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
            move |_e: web_sys::Event| {
                let size = get_window_size();
                window.set_inner_size(size)
            },
        )
            as Box<dyn FnMut(_)>);

        client_window
            .add_event_listener_with_callback(
                "resize",
                closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        closure.forget();
    }

    let mut input = WinitInputHelper::new();

    let mut pixels = {
        let window_size = window.inner_size();

        let surface_texture = SurfaceTexture::new(
            window_size.width,
            window_size.height,
            window.as_ref(),
        );

        Pixels::new_async(WIDTH as _, HEIGHT as _, surface_texture)
            .await
            .unwrap()
    };

    let text_engine = TextEngine::default();

    let raytracer =
        Raytracer::new(pixels.device(), WIDTH as _, RAYTRACER_HEIGHT as _);

    let raytracer_scaler = ScalingTextureRenderer::new(
        pixels.device(),
        &raytracer.output_texture(),
        pixels.render_texture_format(),
        [1.0, (RAYTRACER_HEIGHT as f32) / (HEIGHT as f32)],
    );

    let mut objects = [Object::default(); sc::MAX_OBJECTS as _];

    objects[0] = Object {
        pos: vec4(-4.0, 0.0, 0.0, 2.0),
        color: vec4(1.0, 0.0, 0.0, 1.0),
        ..Default::default()
    };

    objects[1] = Object {
        pos: vec4(4.0, 0.0, 0.0, 2.0),
        color: vec4(0.0, 1.0, 0.0, 1.0),
        ..Default::default()
    };

    objects[2] = Object {
        pos: vec4(0.0, 0.0, 3.0, 2.0),
        color: vec4(0.0, 0.0, 1.0, 1.0),
        ..Default::default()
    };

    let mut camera = Camera::new(
        vec3(0.0, 1.0, -8.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, -1.0, 0.0),
        1.0,
        vec2(WIDTH as _, RAYTRACER_HEIGHT as _),
    );

    let world = World::new(objects, 3);

    let mut surface_size = window.inner_size();

    let mut time_of_last_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if time_of_last_update.elapsed().as_secs_f32() >= SPF {
                time_of_last_update = Instant::now();

                // TODO: Add delta

                if input.key_held(VirtualKeyCode::Up) {
                    camera.camera_origin.z += 1.0;
                }

                if input.key_held(VirtualKeyCode::Down) {
                    camera.camera_origin.z -= 1.0;
                }

                if input.key_held(VirtualKeyCode::Left) {
                    camera.camera_origin.x -= 1.0;
                }

                if input.key_held(VirtualKeyCode::Right) {
                    camera.camera_origin.x += 1.0;
                }

                if input.key_held(VirtualKeyCode::W) {
                    camera.camera_origin.y += 1.0;
                }

                if input.key_held(VirtualKeyCode::S) {
                    camera.camera_origin.y -= 1.0;
                }

                app.draw(Canvas::new(&text_engine, pixels.get_frame_mut()));

                pixels
                    .render_with(|encoder, view, context| {
                        // Draw UI
                        context.scaling_renderer.render(encoder, view);

                        // Draw raytracer
                        raytracer.render(
                            &world,
                            &camera,
                            &context.queue,
                            encoder,
                        );
                        raytracer_scaler.render(encoder, view, surface_size);

                        Ok(())
                    })
                    .unwrap();
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(new_surface_size) = input.window_resized() {
                surface_size = new_surface_size;

                pixels.resize_surface(surface_size.width, surface_size.height);
            }

            app.update();
            window.request_redraw();
        }
    });
}
