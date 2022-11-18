use bevy::prelude::*;
use doome_raytracer as rt;
use glam::vec3;

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    // TODO rgb to srgb?
    pub fn into_vec3(self) -> Vec3 {
        vec3(self.r, self.g, self.b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Health {
    pub val: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Light {
    pub intensity: f32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
pub struct Camera {
    pub origin: Vec3,
    pub look_at: Vec3,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Floor {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Ceiling {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Wall {
    pub x1: i32,
    pub z1: i32,
    pub x2: i32,
    pub z2: i32,
    pub rot: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component)]
pub struct ModelName {
    pub(crate) name: &'static str,
}

impl ModelName {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
pub struct Material {
    pub alpha: Option<f32>,
    pub color: Option<Color>,
    pub reflectivity: Option<f32>,
    pub reflection_color: Option<Color>,
    pub is_textured: Option<bool>,
}

impl Material {
    pub fn with_alpha(mut self, val: f32) -> Self {
        self.alpha = Some(val);
        self
    }

    pub fn with_color(mut self, val: Color) -> Self {
        self.color = Some(val);
        self
    }

    pub fn with_reflectivity(mut self, val: f32) -> Self {
        self.reflectivity = Some(val);
        self
    }

    pub fn with_reflection_color(mut self, val: Color) -> Self {
        self.reflection_color = Some(val);
        self
    }

    pub fn with_texture(mut self) -> Self {
        self.is_textured = Some(true);
        self
    }

    pub fn without_texture(mut self) -> Self {
        self.is_textured = Some(false);
        self
    }

    pub(crate) fn merge_with(self, other: Self) -> Self {
        Self {
            alpha: self.alpha.or(other.alpha),
            color: self.color.or(other.color),
            reflectivity: self.reflectivity.or(other.reflectivity),
            reflection_color: self.reflection_color.or(other.reflection_color),
            is_textured: self.is_textured.or(other.is_textured),
        }
    }

    pub(crate) fn materialize(self) -> rt::Material {
        let color = self.color.unwrap_or_default().into_vec3();
        let texture = self.is_textured.unwrap_or_default();
        let reflectivity = self.reflectivity.unwrap_or_default();
        let reflection_color =
            self.reflection_color.unwrap_or_default().into_vec3();

        rt::Material::default()
            .with_color(color)
            .with_texture(texture)
            .with_reflectivity(reflectivity, reflection_color)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Component)]
pub enum GeometryType {
    Static,
    Dynamic,
}

#[derive(Component)]
pub(crate) struct Synced;
