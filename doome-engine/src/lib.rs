mod canvas;
mod geometry_indexer;
mod pipeline;
mod scaling_texture_renderer;

use std::f32::consts::PI;
use std::io::Cursor;
use std::rc::Rc;

use doome_raytracer::Raytracer;
use doome_raytracer_shader_common as sc;
use doome_surface::Color;
use doome_text::TextEngine;
use glam::{vec2, vec3, Quat, Vec3Swizzles};
use instant::Instant;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub use self::canvas::*;
use self::geometry_indexer::*;
use self::scaling_texture_renderer::*;
use crate::pipeline::Pipeline;

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

    window.set_cursor_visible(false);

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

    let raytracer = Raytracer::new(
        pixels.device(),
        pixels.queue(),
        WIDTH as _,
        RAYTRACER_HEIGHT as _,
    );

    let raytracer_scaler = ScalingTextureRenderer::new(
        pixels.device(),
        &raytracer.output_texture_view(),
        pixels.render_texture_format(),
        [1.0, (RAYTRACER_HEIGHT as f32) / (HEIGHT as f32)],
    );

    // -----

    let mut camera = sc::Camera::new(
        vec3(0.0, 1.0, -3.0),
        vec3(0.0, 1.0, 2.0),
        vec3(0.0, -1.0, 0.0),
        1.0,
        vec2(WIDTH as _, RAYTRACER_HEIGHT as _),
        PI / 2.0,
    );

    // -----

    let mut materials = sc::Materials::default();

    let mat_floor = materials.push(sc::Material::default());

    let mat_wall = materials.push(sc::Material::default());

    let mat_ceiling = materials.push(sc::Material::default());

    let mat_sphere = materials.push(
        sc::Material::default()
            // .with_color(0xff0000)
            .with_reflectivity(0.65, 0xffffff),
    );

    // -----

    let mut geometry = sc::Geometry::default();

    Pipeline::load_model("referenceCube.obj").unwrap();

    // let mut reader =
    //     Cursor::new(include_bytes!("../../models/referenceCube.obj"));
    // let (models, model_materials) =
    //     tobj::load_obj_buf(&mut reader, &Default::default(), |mat| {
    //         let mat = Path::from("../../models").join(mat);
    //         log::info!("Loading material from {}", mat.display());

    //         tobj::load_mtl(mat)
    //     })
    //     .unwrap();

    // log::info!("Models: {models:?}");
    // log::info!("Materials: {model_materials:?}");

    geometry.push_floor(-3, -3, 3, 3, mat_floor);
    geometry.push_wall(-3, 3, -1, 3, 0, mat_wall);
    geometry.push_wall(1, 3, 3, 3, 0, mat_wall);
    geometry.push_wall(3, 3, 3, -3, 1, mat_wall);
    geometry.push_wall(-3, -3, 3, -3, 2, mat_wall);
    geometry.push_wall(-3, -3, -3, 3, 3, mat_wall);

    geometry.push_floor(-1, 3, 1, 5, mat_floor);
    geometry.push_wall(-1, 5, 1, 5, 0, mat_wall);
    geometry.push_wall(1, 3, 1, 5, 1, mat_wall);
    geometry.push_wall(-1, 3, -1, 5, 3, mat_wall);
    geometry.push_icosphere(0, 2, mat_sphere);
    geometry.push_icosphere(-2, 1, mat_sphere);
    geometry.push_icosphere(2, 1, mat_sphere);

    geometry.push_ceiling(-10, -10, 10, 10, mat_ceiling);

    // -----

    let mut geometry_indexer = GeometryIndexer::default();

    for (triangle_id, triangle) in geometry.iter() {
        geometry_indexer.add(triangle, triangle_id);
    }

    let geometry_index = geometry_indexer.build();

    // -----

    let mut lights = sc::Lights::default();

    lights.push(
        sc::Light::new(vec3(-2.5 * 2.0, 3.0, -2.5 * 2.0)).with_intensity(0.7),
    );
    lights.push(
        sc::Light::new(vec3(2.5 * 2.0, 3.0, -2.5 * 2.0)).with_intensity(0.7),
    );

    // -----

    let mut surface_size = window.inner_size();
    let mut time_of_last_update = Instant::now();
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();

    let window_middle =
        PhysicalPosition::new(surface_size.width / 2, surface_size.height / 2);

    let mut track_mouse = true;
    let mut mouse_diff = (0.0, 0.0);

    log::info!("Statistics:");
    log::info!("- triangles: {}", geometry.len());

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            #[cfg(target_arch = "wasm32")]
            let can_redraw = true;

            #[cfg(not(target_arch = "wasm32"))]
            let can_redraw = time_of_last_update.elapsed().as_secs_f32() >= SPF;

            if can_redraw {
                if fps_timer.elapsed().as_secs_f32() >= 1.0 {
                    log::error!("fps = {}", fps_counter);

                    fps_counter = 0;
                    fps_timer = Instant::now();
                }

                app.update();

                time_of_last_update = Instant::now();

                let mouse_delta = mouse_diff;
                mouse_diff = (0.0, 0.0);

                camera.update(|origin, look_at, up| {
                    let dir = *look_at - *origin;

                    const MOUSE_ROTATION_SENSITIVITY: f32 = 0.001;

                    let rot = Quat::from_axis_angle(
                        up.normalize(),
                        // For some reason the up direction here is negative, hence the minus sign
                        -MOUSE_ROTATION_SENSITIVITY * mouse_delta.0,
                    );

                    let new_dir = rot * dir;
                    let new_look_at = *origin + new_dir;

                    look_at.x = new_look_at.x;
                    look_at.z = new_look_at.z;
                });

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

                if input.key_held(VirtualKeyCode::Q)
                    || input.key_held(VirtualKeyCode::E)
                {
                    let sign = if input.key_held(VirtualKeyCode::Q) {
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
                            &geometry_index,
                            &lights,
                            &materials,
                            &context.queue,
                            encoder,
                        );
                        raytracer_scaler.render(encoder, view, surface_size);

                        Ok(())
                    })
                    .unwrap();

                fps_counter += 1;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // TODO: Very hacky, essentially we only track every other mouse position update
            //       so in theory we should get only the updates that are not the result of `set_cursor_position`
            //       if you're wondering why we update the mouse_diff in both cases, the answer is I don't know
            //       for some reason it just feels smoother that way
            let diff = input.mouse_diff();
            if diff != (0.0, 0.0) {
                if track_mouse {
                    mouse_diff.0 -= diff.0;
                    mouse_diff.1 -= diff.1;
                    track_mouse = false;
                } else {
                    mouse_diff.0 += diff.0;
                    mouse_diff.1 += diff.1;
                    window.set_cursor_position(window_middle).ok();
                    track_mouse = true;
                }
            }

            if let Some(new_surface_size) = input.window_resized() {
                surface_size = new_surface_size;

                pixels.resize_surface(surface_size.width, surface_size.height);
            }

            window.request_redraw();
        }
    });
}

trait GeometryExt {
    fn push(&mut self, tri: sc::Triangle);

    fn map_coords(&self, x: i32, z: i32) -> (f32, f32) {
        let x = (x as f32) * 2.0;
        let z = (z as f32) * 2.0;

        (x, z)
    }

    fn push_floor(
        &mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        mat: sc::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = self.map_coords(x1, z1);
        let (x2, z2) = self.map_coords(x2, z2);

        self.push(sc::Triangle::new(
            vec3(x2, 0.0, z1),
            vec3(x1, 0.0, z1),
            vec3(x1, 0.0, z2),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            mat,
        ));

        self.push(sc::Triangle::new(
            vec3(x2, 0.0, z1),
            vec3(x1, 0.0, z2),
            vec3(x2, 0.0, z2),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            mat,
        ));
    }

    fn push_ceiling(
        &mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        mat: sc::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = self.map_coords(x1, z1);
        let (x2, z2) = self.map_coords(x2, z2);

        self.push(sc::Triangle::new(
            vec3(x2, 4.0, z1),
            vec3(x1, 4.0, z2),
            vec3(x1, 4.0, z1),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            mat,
        ));

        self.push(sc::Triangle::new(
            vec3(x2, 4.0, z1),
            vec3(x2, 4.0, z2),
            vec3(x1, 4.0, z2),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            mat,
        ));
    }

    fn push_wall(
        &mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        rot: u8,
        mat: sc::MaterialId,
    ) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let (x1, z1) = self.map_coords(x1, z1);
        let (x2, z2) = self.map_coords(x2, z2);
        let rot = (rot as f32) * (PI / 2.0);

        let vertex = |dx, y, dz| {
            let x = if dx < 0.0 { x1 } else { x2 };
            let z = if dz < 0.0 { z1 } else { z2 };

            vec3(x, y, z)
        };

        self.push(sc::Triangle::new(
            vertex(1.0 * rot.cos(), 0.0, -1.0 * rot.sin()),
            vertex(-1.0 * rot.cos(), 0.0, 1.0 * rot.sin()),
            vertex(-1.0 * rot.cos(), 4.0, 1.0 * rot.sin()),
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            mat,
        ));

        self.push(sc::Triangle::new(
            vertex(1.0 * rot.cos(), 0.0, -1.0 * rot.sin()),
            vertex(-1.0 * rot.cos(), 4.0, 1.0 * rot.sin()),
            vertex(1.0 * rot.cos(), 4.0, -1.0 * rot.sin()),
            vec2(0.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
            mat,
        ));
    }

    // TODO temporary
    fn push_icosphere(&mut self, x: i32, z: i32, mat: sc::MaterialId) {
        let (x, z) = self.map_coords(x, z);
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
                    .map(|vertex| vertex + vec3(x, 1.0, z))
                    .collect();

                self.push(sc::Triangle::new(
                    vertices[0],
                    vertices[1],
                    vertices[2],
                    vec2(0.0, 1.0),
                    vec2(1.0, 1.0),
                    vec2(1.0, 0.0),
                    mat,
                ));
            }
        }
    }
}

impl GeometryExt for sc::Geometry {
    fn push(&mut self, tri: sc::Triangle) {
        sc::Geometry::push(self, tri)
    }
}
