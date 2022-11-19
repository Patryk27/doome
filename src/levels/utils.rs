use std::f32::consts::PI;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use doome_bevy::assets::model::Model;
use doome_bevy::assets::AssetHandle;
use doome_bevy::components::*;
use doome_bevy::physics::{Collider, LineCollider};
use glam::{vec2, vec3};

pub trait LevelBuilderExt<'w, 's> {
    fn floor(&mut self, x1: i32, z1: i32, x2: i32, z2: i32);
    fn ceiling(&mut self, x1: i32, z1: i32, x2: i32, z2: i32);
    fn wall(&mut self, x1: i32, z1: i32, x2: i32, z2: i32, rot: u8);

    fn point_light<'a>(
        &'a mut self,
        x: f32,
        y: f32,
        z: f32,
        r: f32,
        g: f32,
        b: f32,
    ) -> EntityCommands<'w, 's, 'a>;

    fn spot_light<'a>(
        &'a mut self,
        pos: Vec3,
        orientation: Vec3,
        angle: f32,
        color: Vec3,
    ) -> EntityCommands<'w, 's, 'a>;

    fn model<'a>(
        &'a mut self,
        handle: AssetHandle<Model>,
    ) -> ModelBuilder<'w, 's, 'a>;
}

impl<'w, 's> LevelBuilderExt<'w, 's> for Commands<'w, 's> {
    fn floor(&mut self, x1: i32, z1: i32, x2: i32, z2: i32) {
        self.spawn((GeometryType::Static, Floor { x1, z1, x2, z2 }));
    }

    fn ceiling(&mut self, x1: i32, z1: i32, x2: i32, z2: i32) {
        self.spawn((GeometryType::Static, Ceiling { x1, z1, x2, z2 }));
    }

    fn wall(&mut self, x1: i32, z1: i32, x2: i32, z2: i32, rot: u8) {
        const HEIGHT: f32 = 4.0;

        let angle = (rot as f32) * (PI / 2.0);

        let center = {
            let (x1, x2) = (x1.min(x2), x1.max(x2));
            let (z1, z2) = (z1.min(z2), z1.max(z2));
            let (x1, z1) = (x1 as f32, z1 as f32);
            let (x2, z2) = (x2 as f32, z2 as f32);
            let center = vec3((x1 + x2) / 2.0, HEIGHT / 2.0, (z1 + z2) / 2.0);

            center
        };

        let collider_start = vec2(x1 as f32, z1 as f32);
        let collider_end = vec2(x2 as f32, z2 as f32);

        // TODO: Walls are weird right now, because we're giving the geometry and colliders here and absolute coordinates
        //       as opposed to coordinates in relation to the transform. As such, walls right now have the identity transform.
        let transform = Transform::IDENTITY;
        // Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, angle))
        //     .with_translation(center);

        self.spawn((
            GeometryType::Static,
            Wall {
                x1,
                z1,
                x2,
                z2,
                rot,
            },
            Collider::Line(LineCollider {
                start: collider_start,
                end: collider_end,
            }),
            transform,
        ));
    }

    fn point_light<'a>(
        &'a mut self,
        x: f32,
        y: f32,
        z: f32,
        r: f32,
        g: f32,
        b: f32,
    ) -> EntityCommands<'w, 's, 'a> {
        self.spawn((
            Light {
                enabled: true,
                intensity: 1.0,
                kind: LightKind::Point,
            },
            Transform::from_xyz(x, y, z),
            Color { r, g, b },
        ))
    }

    fn spot_light<'a>(
        &'a mut self,
        pos: Vec3,
        point_at: Vec3,
        angle: f32,
        color: Vec3,
    ) -> EntityCommands<'w, 's, 'a> {
        self.spawn((
            Light {
                enabled: true,
                intensity: 1.0,
                kind: LightKind::Spot { point_at, angle },
            },
            Transform::from_translation(pos),
            Color::from_vec3(color),
        ))
    }

    fn model<'a>(
        &'a mut self,
        handle: AssetHandle<Model>,
    ) -> ModelBuilder<'w, 's, 'a> {
        ModelBuilder::new(self, handle)
    }
}

pub struct ModelBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    handle: AssetHandle<Model>,
    geo_type: GeometryType,
    transform: Transform,
    material: Option<Material>,
}

impl<'w, 's, 'a> ModelBuilder<'w, 's, 'a> {
    fn new(
        commands: &'a mut Commands<'w, 's>,
        handle: AssetHandle<Model>,
    ) -> Self {
        Self {
            commands,
            handle,
            geo_type: GeometryType::Static,
            transform: Default::default(),
            material: Default::default(),
        }
    }

    pub fn dynamic(mut self) -> Self {
        self.geo_type = GeometryType::Dynamic;
        self
    }

    pub fn with_translation(mut self, val: Vec3) -> Self {
        self.transform.translation = val;
        self
    }

    pub fn with_rotation(mut self, val: Quat) -> Self {
        self.transform.rotation = val;
        self
    }

    pub fn with_scale(mut self, val: Vec3) -> Self {
        self.transform.scale = val;
        self
    }

    pub fn with_material(mut self, val: Material) -> Self {
        self.material = Some(val);
        self
    }

    pub fn spawn(self) -> EntityCommands<'w, 's, 'a> {
        let mut ec =
            self.commands
                .spawn((self.handle, self.transform, self.geo_type));

        if let Some(material) = self.material {
            ec.insert(material);
        }

        ec
    }
}
