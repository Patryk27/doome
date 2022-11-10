mod canvas;

use std::rc::Rc;

use doome_raytracer::Raytracer;
use doome_raytracer_shader_common as sc;
use doome_surface::Color;
use doome_text::TextEngine;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub use self::canvas::*;

pub const WIDTH: u16 = 320;
pub const HEIGHT: u16 = 200;

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
        env_logger::init();
        pollster::block_on(run(app));
    }
}

async fn run(mut app: impl App + 'static) {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);

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
    let raytracer = Raytracer::new(&pixels);

    let mut sc_context = sc::Context {
        screen_width: window.inner_size().width as f32,
        screen_height: window.inner_size().height as f32,
        object_count: 3,
    };

    let sc_objects = vec![
        sc::Object {
            center_x: 0.3,
            center_y: 0.4,
            center_z: 0.1,
            radius: 0.4,
            color_r: 1.0,
            color_g: 0.0,
            color_b: 0.0,
        },
        sc::Object {
            center_x: 0.5,
            center_y: 0.4,
            center_z: 0.0,
            radius: 0.4,
            color_r: 0.0,
            color_g: 1.0,
            color_b: 0.0,
        },
        sc::Object {
            center_x: 0.7,
            center_y: 0.4,
            center_z: 0.1,
            radius: 0.4,
            color_r: 0.0,
            color_g: 0.0,
            color_b: 1.0,
        },
    ];

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            app.draw(Canvas::new(&text_engine, pixels.get_frame_mut()));

            pixels
                .render_with(|encoder, view, context| {
                    context.scaling_renderer.render(encoder, view);

                    raytracer.render(
                        &context.queue,
                        encoder,
                        view,
                        &sc_context,
                        &sc_objects,
                    );

                    Ok(())
                })
                .unwrap();
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                sc_context.screen_width = size.width as _;
                sc_context.screen_height = size.height as _;

                pixels.resize_surface(size.width, size.height);
            }

            app.update();
            window.request_redraw();
        }
    });
}
