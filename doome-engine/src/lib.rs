mod canvas;
mod scaling_texture_renderer;

use std::f32::consts::PI;
use std::io::Cursor;
use std::rc::Rc;

use doome_raytracer::Raytracer;
use doome_raytracer_shader_common as sc;
use doome_surface::Color;
use doome_text::TextEngine;
use glam::{vec2, vec3, Vec3, Vec3Swizzles};
use instant::Instant;
use pixels::{Pixels, SurfaceTexture};
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
        &raytracer.output_texture_view(),
        pixels.render_texture_format(),
        [1.0, (RAYTRACER_HEIGHT as f32) / (HEIGHT as f32)],
    );

    // -----

    let mut camera = sc::Camera::new(
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 5.0),
        vec3(0.0, -1.0, 0.0),
        1.0,
        vec2(WIDTH as _, RAYTRACER_HEIGHT as _),
    );

    // -----

    let mut materials = sc::Materials::default();

    let m1 = materials.push(
        sc::Material::default()
            .with_color(0x1a1417)
            .with_reflectivity(0.5, 0xffffff),
    );
    let m2 = materials.push(
        sc::Material::default()
            .with_color(0x505050)
            .with_reflectivity(1.0, 0xffffff),
    );
    let m3 = materials.push(sc::Material::default().with_color(0x8db35e));
    let m4 = materials.push(sc::Material::default().with_color(0x384d64));

    // -----

    let mut geometry = sc::Geometry::default();

    geometry.push(sc::Triangle::new(
        vec3(-10.0, 0.0, 10.0),
        vec3(10.0, 0.0, 10.0),
        vec3(10.0, 0.0, -10.0),
        m1,
    ));

    geometry.push(sc::Triangle::new(
        vec3(-10.0, 0.0, -10.0),
        vec3(-10.0, 0.0, 10.0),
        vec3(10.0, 0.0, -10.0),
        m1,
    ));

    push_icosphere(&mut geometry, vec3(0.0, 1.0, 5.0), m2);
    push_icosphere(&mut geometry, vec3(-2.0, 1.0, 4.0), m3);
    push_icosphere(&mut geometry, vec3(2.0, 1.0, 4.0), m4);

    // -----

    let mut lights = sc::Lights::default();

    lights.push(sc::Light::new(vec3(0.0, 5.0, -2.0), 0xffffff));
    lights.push(sc::Light::new(vec3(0.0, 3.0, 4.0), 0xffffff));

    // -----

    let mut surface_size = window.inner_size();
    let mut time_of_last_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            #[cfg(target_arch = "wasm")]
            let can_redraw = true;

            #[cfg(not(target_arch = "wasm"))]
            let can_redraw = time_of_last_update.elapsed().as_secs_f32() >= SPF;

            if can_redraw {
                app.update();

                time_of_last_update = Instant::now();

                // TODO: Add delta

                if input.key_held(VirtualKeyCode::W)
                    || input.key_held(VirtualKeyCode::S)
                {
                    let sign = if input.key_held(VirtualKeyCode::W) {
                        1.0
                    } else {
                        -1.0
                    };

                    camera.update(|origin, look_at, _| {
                        let dist = *look_at - *origin;

                        *origin += sign * dist * 0.025;
                        *look_at += sign * dist * 0.025;
                    });
                }

                if input.key_held(VirtualKeyCode::A)
                    || input.key_held(VirtualKeyCode::D)
                {
                    let sign = if input.key_held(VirtualKeyCode::A) {
                        -1.0
                    } else {
                        1.0
                    };

                    camera.update(|origin, look_at, _| {
                        let dir = look_at.xz() - origin.xz();
                        let dir_len = dir.length();
                        let dir_angle = dir.angle_between(vec2(0.0, 1.0));
                        let dir_angle = dir_angle + 0.025 * sign;

                        let new_dir = vec2(dir_len, dir_len)
                            * vec2(dir_angle.sin(), dir_angle.cos());

                        let new_look_at = origin.xz() + new_dir;

                        look_at.x = new_look_at.x;
                        look_at.z = new_look_at.y;
                    });
                }

                if input.key_held(VirtualKeyCode::Q)
                    || input.key_held(VirtualKeyCode::E)
                {
                    let sign = if input.key_held(VirtualKeyCode::Q) {
                        1.0
                    } else {
                        -1.0
                    };

                    camera.update(|origin, look_at, _| {
                        let dir = look_at.xz() - origin.xz();
                        let dir_angle = dir.angle_between(vec2(0.0, 1.0));
                        let dir_angle_perpendicular = dir_angle + PI / 2.0 + PI;

                        let delta = vec2(
                            dir_angle_perpendicular.sin(),
                            dir_angle_perpendicular.cos(),
                        ) * 0.1
                            * sign;

                        origin.x += delta.x;
                        origin.z += delta.y;

                        look_at.x += delta.x;
                        look_at.z += delta.y;
                    });
                }

                if input.key_held(VirtualKeyCode::R)
                    || input.key_held(VirtualKeyCode::F)
                {
                    let sign = if input.key_held(VirtualKeyCode::R) {
                        1.0
                    } else {
                        -1.0
                    };

                    camera.update(|origin, look_at, _| {
                        origin.y += sign * 0.2;
                        look_at.y += sign * 0.2;
                    });
                }

                app.draw(Canvas::new(&text_engine, pixels.get_frame_mut()));

                pixels
                    .render_with(|encoder, view, context| {
                        // Draw UI
                        context.scaling_renderer.render(encoder, view);

                        // Draw raytracer
                        raytracer.render(
                            &camera,
                            &geometry,
                            &lights,
                            &materials,
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

            window.request_redraw();
        }
    });
}

// TODO temporary
fn push_icosphere(geometry: &mut sc::Geometry, pos: Vec3, mat: sc::MaterialId) {
    let mut reader = Cursor::new(include_bytes!("../../icosphere.obj"));

    let (models, _) =
        tobj::load_obj_buf(&mut reader, &Default::default(), |_| todo!())
            .unwrap();

    for model in models {
        let mesh = &model.mesh;

        for indices in mesh.indices.chunks(3) {
            let vertices: Vec<_> = indices
                .iter()
                .copied()
                .map(|index| {
                    let index = index as usize;

                    vec3(
                        mesh.positions[3 * index],
                        mesh.positions[3 * index + 1],
                        mesh.positions[3 * index + 2],
                    )
                })
                .map(|vertex| pos + vertex)
                .collect();

            geometry.push(sc::Triangle::new(
                vertices[0],
                vertices[1],
                vertices[2],
                mat,
            ));
        }
    }
}
