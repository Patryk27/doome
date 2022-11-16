use std::f32::consts::PI;

use bevy::app::AppExit;
use bevy::diagnostic::{
    Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use doome_bevy::doome::{DoomePlugin, DoomeRenderInit, DoomeRendererContext};
use doome_bevy::pixels_plugin::{PixelsPlugin, PixelsState};
use doome_bevy::text::Text;
use doome_engine::pipeline::PipelineBuilder;
use doome_engine::{
    Canvas, GeometryBuilder, HEIGHT, HUD_HEIGHT, RAYTRACER_HEIGHT, WIDTH,
};
use doome_raytracer as rt;
use doome_surface::Color;
use glam::{vec2, vec3, Vec3Swizzles};

// TODO: Right now we're including files like .gitignore or *.blend (and the pesky *.blend1)
//       ideally we'd remove them before including them in the binary. Perhaps a custom proc macro?
const ASSETS: include_dir::Dir = include_dir::include_dir!("assets");

fn main() {
    let camera = rt::Camera::new(
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

    // -----

    let mut monke_xform = rt::math::identity();
    rt::math::translate(&mut monke_xform, vec3(0.0, 1.0, 0.0));
    rt::math::rotate(&mut monke_xform, 45.0, vec3(0.0, 1.0, 0.0));

    let mut ref_cube_xform = rt::math::identity();
    rt::math::translate(&mut ref_cube_xform, vec3(2.0, 1.0, 0.0));

    // -----

    let mut pipeline = PipelineBuilder::new(ASSETS);

    let monke_mesh = pipeline.load_model("monke.obj", mat_monke).unwrap();
    let reference_cube = pipeline
        .load_model("referenceCube.obj", mat_reference_cube)
        .unwrap();
    let diamond_mesh = pipeline.load_model("diamond.obj", mat_diamond).unwrap();

    let pipeline = pipeline.build();

    // -----

    let mut static_geometry = GeometryBuilder::default();

    static_geometry.push_floor(-3, -3, 3, 3, mat_matte);
    static_geometry.push_wall(-3, 3, -1, 3, 0, mat_matte);
    static_geometry.push_wall(1, 3, 3, 3, 0, mat_matte);
    static_geometry.push_wall(3, 3, 3, -3, 1, mat_matte);
    static_geometry.push_wall(-3, -3, 3, -3, 2, mat_matte);
    static_geometry.push_wall(-3, -3, -3, 3, 3, mat_matte);

    static_geometry.push_floor(-1, 3, 1, 5, mat_matte);
    static_geometry.push_wall(-1, 5, 1, 5, 0, mat_matte);
    static_geometry.push_wall(1, 3, 1, 5, 1, mat_matte);
    static_geometry.push_wall(-1, 3, -1, 5, 3, mat_matte);

    static_geometry.push_icosphere(0, 2, mat_sphere);
    static_geometry.push_icosphere(-2, 1, mat_sphere);
    static_geometry.push_icosphere(2, 1, mat_sphere);

    static_geometry.push_ceiling(-10, -10, 10, 10, mat_matte);

    // -----

    let mut lights = rt::Lights::default();

    lights.push(
        rt::Light::new(vec3(-2.5 * 2.0, 3.0, -2.5 * 2.0)).with_intensity(0.7),
    );
    lights.push(
        rt::Light::new(vec3(2.5 * 2.0, 3.0, -2.5 * 2.0)).with_intensity(0.7),
    );

    // -----

    pipeline.insert_to_geometry(
        monke_mesh,
        &mut static_geometry,
        monke_xform,
        1.0,
    );
    pipeline.insert_to_geometry(
        reference_cube,
        &mut static_geometry,
        ref_cube_xform,
        0.5,
    );
    pipeline.insert_to_geometry(
        diamond_mesh,
        &mut static_geometry,
        rt::math::translated(vec3(-3.0, 1.0, -1.0)),
        1.0,
    );

    // -----

    let (static_geometry, static_geometry_mapping) = static_geometry.build();
    let static_geometry_index = rt::GeometryIndexer::index(&static_geometry);

    // -----

    // TODO: Add FPS limiting
    App::new()
        .insert_resource(DoomeRenderInit { pipeline })
        .insert_resource(DoomeRendererContext {
            camera,
            static_geometry,
            static_geometry_mapping,
            static_geometry_index,
            lights,
            materials,
        })
        .insert_resource(Text::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_plugin(DoomePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_system(update_camera)
        .add_system(quit_on_exit)
        .add_system(render_ui)
        .add_startup_system(hide_cursor)
        .run();
}

fn hide_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_grab_mode(CursorGrabMode::Confined);
    window.set_cursor_visibility(false);
}

fn quit_on_exit(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn render_ui(
    mut pixels: ResMut<PixelsState>,
    text: Res<Text>,
    diagnostics: Res<Diagnostics>,
) {
    let frame = pixels.inner_mut().get_frame_mut();
    let mut canvas = Canvas::new(&text.text_engine, frame);

    canvas.rect(
        0,
        HEIGHT - HUD_HEIGHT,
        WIDTH,
        HEIGHT,
        Color {
            r: 0x10,
            g: 0x10,
            b: 0x10,
            a: 0xff,
        },
    );

    let fps_diagnostic =
        diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();

    if let Some(fps) = fps_diagnostic.smoothed() {
        canvas.text(
            10,
            HEIGHT - 30,
            format!("FPS: {fps:>.6}{}", fps_diagnostic.suffix),
        );
    }
}

fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut context: ResMut<DoomeRendererContext>,
) {
    let camera = &mut context.camera;
    let delta = time.delta_seconds();

    const MOUSE_ROTATION_SENSITIVITY: f32 = 0.5;
    const PLANAR_MOVEMENT_SPEED: f32 = 10.0;
    const CELESTIAL_MOVEMENT_SPEED: f32 = 4.0;
    const ROTATION_SPEED: f32 = 2.0;

    for ev in mouse_motion.iter() {
        camera.update(|origin, look_at, up| {
            let dir = *look_at - *origin;

            let rot = Quat::from_axis_angle(
                up.normalize(),
                // For some reason the up direction here is negative, hence the minus sign
                -MOUSE_ROTATION_SENSITIVITY * ev.delta.x * delta,
            );

            let new_dir = rot * dir;
            let new_look_at = *origin + new_dir;

            look_at.x = new_look_at.x;
            look_at.z = new_look_at.z;
        });
    }

    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::S) {
        let sign = if keys.pressed(KeyCode::W) { 1.0 } else { -1.0 };

        camera.update(|origin, look_at, _| {
            let dist = (*look_at - *origin).normalize();

            *origin += sign * dist * PLANAR_MOVEMENT_SPEED * delta;
            *look_at += sign * dist * PLANAR_MOVEMENT_SPEED * delta;
        });
    }

    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::D) {
        let sign = if keys.pressed(KeyCode::A) { 1.0 } else { -1.0 };

        camera.update(|origin, look_at, _| {
            let dir = look_at.xz() - origin.xz();
            let dir_angle = dir.angle_between(vec2(0.0, 1.0));
            let dir_angle_perpendicular = dir_angle + PI / 2.0 + PI;

            let movement = vec2(
                dir_angle_perpendicular.sin(),
                dir_angle_perpendicular.cos(),
            )
            .normalize()
                * PLANAR_MOVEMENT_SPEED
                * sign
                * delta;

            origin.x += movement.x;
            origin.z += movement.y;

            look_at.x += movement.x;
            look_at.z += movement.y;
        });
    }

    if keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::E) {
        let sign = if keys.pressed(KeyCode::Q) { -1.0 } else { 1.0 };

        camera.update(|origin, look_at, _| {
            let dir = look_at.xz() - origin.xz();
            let dir_len = dir.length();
            let dir_angle = dir.angle_between(vec2(0.0, 1.0));
            let dir_angle = dir_angle + ROTATION_SPEED * sign * delta;

            let new_dir =
                vec2(dir_len, dir_len) * vec2(dir_angle.sin(), dir_angle.cos());

            let new_look_at = origin.xz() + new_dir;

            look_at.x = new_look_at.x;
            look_at.z = new_look_at.y;
        });
    }

    if keys.pressed(KeyCode::R) || keys.pressed(KeyCode::F) {
        let sign = if keys.pressed(KeyCode::R) { 1.0 } else { -1.0 };

        camera.update(|origin, look_at, _| {
            origin.y += sign * CELESTIAL_MOVEMENT_SPEED * delta;
            look_at.y += sign * CELESTIAL_MOVEMENT_SPEED * delta;
        });
    }
}
