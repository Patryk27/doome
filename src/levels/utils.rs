use std::f32::consts::PI;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use doome_bevy::assets::{AssetHandle, Assets, Model};
use doome_bevy::components::*;
use doome_bevy::nav::NavObstacle;
use doome_bevy::physics::components::Collider;
use glam::{vec2, vec3};

pub struct LevelBuilder<'p, 'w, 's> {
    commands: &'p mut Commands<'w, 's>,
    assets: &'p Assets,
}

impl<'p, 'w, 's> LevelBuilder<'p, 'w, 's> {
    pub fn new(commands: &'p mut Commands<'w, 's>, assets: &'p Assets) -> Self {
        Self { commands, assets }
    }

    pub fn floor<'a>(
        &'a mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
    ) -> LevelModelBuilder<'w, 's, 'a> {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let dx = x2 - x1 + 1;
        let dz = z2 - z1 + 1;

        log::debug!(
            "floor({}, {}, {}, {}); dx={}, dz={}",
            x1,
            z1,
            x2,
            z2,
            dx,
            dz
        );

        assert!(dx > 0 && dz > 0, "Floor has no area");

        self.model("floor")
            .with_translation(vec3(
                (x1 + x2) as f32 / 2.0,
                0.0,
                (z1 + z2) as f32 / 2.0,
            ))
            .with_scale(vec3((dx as f32) / 2.0, 1.0, (dz as f32) / 2.0))
            .with_material(
                Material::default()
                    .with_color(Color::hex(0xffffff))
                    .with_reflectivity(0.1)
                    .with_reflection_color(Color::hex(0xffffff))
                    .with_uv_divisor(dx as _, dz as _),
            )
    }

    pub fn ceiling<'a>(
        &'a mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
    ) -> LevelModelBuilder<'w, 's, 'a> {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let dx = x2 - x1 + 1;
        let dz = z2 - z1 + 1;

        log::debug!(
            "ceiling({}, {}, {}, {}); dx={}, dz={}",
            x1,
            z1,
            x2,
            z2,
            dx,
            dz
        );

        assert!(dx > 0 && dz > 0, "Ceiling has no area");

        self.model("ceiling")
            .with_translation(vec3(
                (x1 + x2) as f32 / 2.0,
                2.99,
                (z1 + z2) as f32 / 2.0,
            ))
            .with_scale(vec3((dx as f32) / 2.0, 1.0, (dz as f32) / 2.0))
            .with_material(
                Material::default()
                    .with_color(Color::hex(0xffffff))
                    .with_reflectivity(0.1)
                    .with_reflection_color(Color::hex(0xffffff))
                    .with_uv_divisor(dx as _, dz as _),
            )
    }

    pub fn wall<'a>(
        &'a mut self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
        rot: u8,
    ) -> LevelModelBuilder<'w, 's, 'a> {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let dx = x2 - x1 + 1;
        let dz = z2 - z1 + 1;

        log::debug!(
            "wall({}, {}, {}, {}, {}); dx={}, dz={}",
            x1,
            z1,
            x2,
            z2,
            rot,
            dx,
            dz
        );

        assert!(dx == 1 || dz == 1, "Wall is not axis-aligned");

        let extrude = match rot {
            0 => vec3(0.0, 0.0, 0.5),
            1 => vec3(0.5, 0.0, 0.0),
            2 => vec3(0.0, 0.0, -0.5),
            3 => vec3(-0.5, 0.0, 0.0),
            _ => Default::default(),
        };

        let scale = if dx == 1 { dz } else { dx };

        self.model("wall")
            .obstacle()
            .with_translation(
                vec3(
                    (x1 as f32 + x2 as f32) / 2.0,
                    0.0,
                    (z1 as f32 + z2 as f32) / 2.0,
                ) + extrude,
            )
            .with_rotation(Quat::from_rotation_y(PI / 2.0 * (rot as f32)))
            .with_scale(vec3((scale as f32) / 2.0, 1.0, 1.0))
            .with_material(
                Material::default()
                    .with_color(Color::hex(0xffffff))
                    .with_uv_divisor(scale as _, 1),
            )
            .with_collider(Collider::line(vec2(-1.0, 0.0), vec2(1.0, 0.0)))
    }

    pub fn point_light<'a>(
        &'a mut self,
        x: f32,
        y: f32,
        z: f32,
        c: u32,
    ) -> EntityCommands<'w, 's, 'a> {
        self.commands.spawn((
            Light {
                enabled: true,
                intensity: 1.0,
                kind: LightKind::Point,
            },
            Transform::from_xyz(x, y, z),
            Color::hex(c),
        ))
    }

    pub fn spot_light<'a>(
        &'a mut self,
        pos: Vec3,
        point_at: Vec3,
        angle: f32,
        color: Color,
        intensity: f32,
    ) -> EntityCommands<'w, 's, 'a> {
        self.commands.spawn((
            Light {
                enabled: true,
                intensity,
                kind: LightKind::Spot { point_at, angle },
            },
            Transform::from_translation(pos),
            color,
        ))
    }

    pub fn model<'a>(
        &'a mut self,
        name: &'static str,
    ) -> LevelModelBuilder<'w, 's, 'a> {
        LevelModelBuilder::new(self.commands, self.assets.load_model(name))
    }

    pub fn complete<T>(self, level: T)
    where
        T: Resource,
    {
        self.commands.insert_resource(level);
    }
}

pub struct LevelModelBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    handle: AssetHandle<Model>,
    geo_type: GeometryType,
    transform: Transform,
    material: Option<Material>,
    collider: Option<Collider>,
    is_obstacle: bool,
}

impl<'w, 's, 'a> LevelModelBuilder<'w, 's, 'a> {
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
            collider: Default::default(),
            is_obstacle: false,
        }
    }

    pub fn obstacle(mut self) -> Self {
        self.is_obstacle = true;
        self
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

    pub fn with_collider(mut self, val: Collider) -> Self {
        self.collider = Some(val);
        self
    }

    pub fn spawn(self) -> EntityCommands<'w, 's, 'a> {
        let mut entity =
            self.commands
                .spawn((self.handle, self.transform, self.geo_type));

        if let Some(material) = self.material {
            entity.insert(material);
        }

        if let Some(collider) = self.collider {
            entity.insert(collider);
        }

        if self.is_obstacle {
            entity.insert(NavObstacle);
        }

        entity
    }
}
