mod canvas;
mod geometry_ext;
mod pipeline;
mod scaling_texture_renderer;

use std::f32::consts::PI;
use std::rc::Rc;

use doome_raytracer as rt;
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
use self::geometry_ext::GeometryExt;
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
    fn dir(&self) -> &'static include_dir::Dir<'static>;
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

    // -----

    let mut camera = rt::Camera::new(
        vec3(0.0, 1.0, -3.0),
        vec3(0.0, 1.0, 2.0),
        vec3(0.0, -1.0, 0.0),
        1.0,
        vec2(WIDTH as _, RAYTRACER_HEIGHT as _),
        PI / 2.0,
    );

    // -----

    let mut materials = rt::Materials::default();

    let mat_monke = materials.push(
        rt::Material::default()
            .with_reflectivity(0.1, 0xffffff)
            .with_texture(true),
    );

    let mat_reference_cube = materials.push(
        rt::Material::default()
            .with_reflectivity(0.0, 0xffffff)
            .with_texture(true),
    );

    let mat_diamond = materials.push(
        rt::Material::default()
            .with_color(0xff0000)
            .with_reflectivity(0.3, 0xff0000)
            .with_texture(true),
    );

    let mat_matte =
        materials.push(rt::Material::default().with_color(0x666666));

    let mat_sphere = materials.push(
        rt::Material::default()
            .with_color(0xff0000)
            .with_reflectivity(0.65, 0xffffff),
    );

    let mut pipeline = Pipeline::builder(app.dir());

    let monke_mesh = pipeline.load_model("monke.obj", mat_monke).unwrap();
    let reference_cube = pipeline
        .load_model("referenceCube.obj", mat_reference_cube)
        .unwrap();
    let diamond_mesh = pipeline.load_model("diamond.obj", mat_diamond).unwrap();

    let pipeline = pipeline.build();

    // -----

    let raytracer = rt::Engine::new(
        pixels.device(),
        pixels.queue(),
        WIDTH as _,
        RAYTRACER_HEIGHT as _,
        pipeline.atlas().as_raw(),
    );

    let raytracer_scaler = ScalingTextureRenderer::new(
        pixels.device(),
        &raytracer.output_texture_view(),
        pixels.render_texture_format(),
        [1.0, (RAYTRACER_HEIGHT as f32) / (HEIGHT as f32)],
    );

    // -----

    let mut monke_xform = rt::math::identity();
    rt::math::translate(&mut monke_xform, vec3(0.0, 1.0, 0.0));
    rt::math::rotate(&mut monke_xform, 45.0, vec3(0.0, 1.0, 0.0));

    let mut ref_cube_xform = rt::math::identity();
    rt::math::translate(&mut ref_cube_xform, vec3(2.0, 1.0, 0.0));

    // -----

    let mut geometry = rt::Geometry::default();

    geometry.push_floor(-3, -3, 3, 3, mat_matte);
    geometry.push_wall(-3, 3, -1, 3, 0, mat_matte);
    geometry.push_wall(1, 3, 3, 3, 0, mat_matte);
    geometry.push_wall(3, 3, 3, -3, 1, mat_matte);
    geometry.push_wall(-3, -3, 3, -3, 2, mat_matte);
    geometry.push_wall(-3, -3, -3, 3, 3, mat_matte);

    geometry.push_floor(-1, 3, 1, 5, mat_matte);
    geometry.push_wall(-1, 5, 1, 5, 0, mat_matte);
    geometry.push_wall(1, 3, 1, 5, 1, mat_matte);
    geometry.push_wall(-1, 3, -1, 5, 3, mat_matte);

    geometry.push_icosphere(0, 2, mat_sphere);
    geometry.push_icosphere(-2, 1, mat_sphere);
    geometry.push_icosphere(2, 1, mat_sphere);

    geometry.push_ceiling(-10, -10, 10, 10, mat_matte);

    let _monke =
        pipeline.insert_to_geometry(monke_mesh, &mut geometry, monke_xform);

    let _ref_cube = pipeline.insert_to_geometry(
        reference_cube,
        &mut geometry,
        ref_cube_xform,
    );

    let _diamond = pipeline.insert_to_geometry(
        diamond_mesh,
        &mut geometry,
        rt::math::translated(vec3(-3.0, 1.0, -1.0)),
    );

    // -----

    let geometry_index = rt::GeometryIndexer::index(&geometry);

    // -----

    let mut lights = rt::Lights::default();

    lights.push(
        rt::Light::new(vec3(-2.5 * 2.0, 3.0, -2.5 * 2.0)).with_intensity(0.7),
    );
    lights.push(
        rt::Light::new(vec3(2.5 * 2.0, 3.0, -2.5 * 2.0)).with_intensity(0.7),
    );

    // -----

    let mut surface_size = window.inner_size();
    let mut time_of_last_update = Instant::now();
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    let mut draw_tt_sum = 0.0;

    let window_middle =
        PhysicalPosition::new(surface_size.width / 2, surface_size.height / 2);

    let mut track_mouse = true;
    let mut mouse_diff = (0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            #[cfg(target_arch = "wasm32")]
            let can_redraw = true;

            #[cfg(not(target_arch = "wasm32"))]
            let can_redraw = time_of_last_update.elapsed().as_secs_f32() >= SPF;

            if can_redraw {
                if fps_timer.elapsed().as_secs_f32() >= 1.0 {
                    let avg_draw_tt = draw_tt_sum / (fps_counter.max(1) as f32);

                    log::debug!(
                        "fps = {}; avg-draw-tt = {} ms",
                        fps_counter,
                        1000.0 * avg_draw_tt,
                    );

                    fps_counter = 0;
                    fps_timer = Instant::now();
                    draw_tt_sum = 0.0;
                }

                // rt::math::rotate(&mut monke_xform, 0.1, vec3(0.0, 1.0, 0.0));

                // pipeline.update_geometry(
                //     monke,
                //     monke_mesh,
                //     &mut geometry,
                //     monke_xform,
                // );

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

                let draw_tt = Instant::now();

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
                draw_tt_sum += draw_tt.elapsed().as_secs_f32();
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
