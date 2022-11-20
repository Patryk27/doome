use std::f32::consts::PI;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use doome_bevy::assets::{AssetHandle, Assets, Model};
use doome_bevy::components::*;
use glam::vec3;

pub struct LevelBuilder<'p, 'w, 's> {
    commands: &'p mut Commands<'w, 's>,
    assets: &'p Assets,
}

impl<'p, 'w, 's> LevelBuilder<'p, 'w, 's> {
    pub fn new(commands: &'p mut Commands<'w, 's>, assets: &'p Assets) -> Self {
        Self { commands, assets }
    }

    pub fn floor(&mut self, x1: i32, z1: i32, x2: i32, z2: i32) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let dx = x2 - x1;
        let dz = z2 - z1;

        // TODO add translation
        // TODO add collider
        self.model("floor")
            .with_scale(vec3(dx as f32, 1.0, dz as f32))
            .with_material(
                Material::default()
                    .with_color(Color::hex(0xffffff))
                    .with_reflectivity(0.1)
                    .with_reflection_color(Color::hex(0xffffff))
                    .with_uv_divisor(dx as _, dz as _),
            )
            .spawn();
    }

    pub fn wall(&mut self, x1: i32, z1: i32, x2: i32, z2: i32, rot: u8) {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (z1, z2) = (z1.min(z2), z1.max(z2));
        let dx = x2 - x1;
        let dz = z2 - z1;

        assert!(dx == 0 || dz == 0);

        let scale = if dx == 0 { dz } else { dx };

        // TODO something's off here in regards to the translation
        // TODO add collider
        self.model("wall")
            .with_scale(vec3(scale as f32, 1.0, 1.0))
            .with_rotation(Quat::from_rotation_y(PI / 2.0 * (rot as f32)))
            .with_translation(vec3(
                (x1 as f32 + x2 as f32) / 2.0,
                0.0,
                (z1 as f32 + z2 as f32) / 2.0,
            ))
            .with_material(
                Material::default()
                    .with_color(Color::hex(0xffffff))
                    .with_uv_divisor(scale as _, 1),
            )
            .spawn();
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
        color: Vec3,
    ) -> EntityCommands<'w, 's, 'a> {
        self.commands.spawn((
            Light {
                enabled: true,
                intensity: 1.0,
                kind: LightKind::Spot { point_at, angle },
            },
            Transform::from_translation(pos),
            Color::from_vec3(color),
        ))
    }

    pub fn model<'a>(
        &'a mut self,
        name: &'static str,
    ) -> LevelModelBuilder<'w, 's, 'a> {
        LevelModelBuilder::new(&mut self.commands, self.assets.load_model(name))
    }
}

pub struct LevelModelBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    handle: AssetHandle<Model>,
    geo_type: GeometryType,
    transform: Transform,
    material: Option<Material>,
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
