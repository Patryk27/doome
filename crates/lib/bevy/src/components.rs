use std::ops;

use bevy::prelude::*;
use doome_raytracer as rt;
use glam::vec3;

use crate::assets::{AssetHandle, Texture};

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn srgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn hex(rgb: u32) -> Self {
        let [_, r, g, b] = rgb.to_be_bytes();
        let convert = |c: u8| (((c as f32) / 255.0 + 0.055) / 1.055).powf(2.4);

        Self::srgb(convert(r), convert(g), convert(b))
    }

    pub fn into_vec3(self) -> Vec3 {
        vec3(self.r, self.g, self.b)
    }

    pub fn into_tuple(self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    pub fn from_vec3(vec: Vec3) -> Self {
        Self {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
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

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Light {
    // I feel like for proper bevy-ness this should be a different component
    // but I like the brevity and simplicity of this solution
    pub enabled: bool,
    pub intensity: f32,
    pub kind: LightKind,
}

impl Light {
    pub fn point_at_mut(&mut self) -> Option<&mut Vec3> {
        match &mut self.kind {
            LightKind::Point => None,
            LightKind::Spot { point_at, .. } => Some(point_at),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LightKind {
    Point,
    Spot { point_at: Vec3, angle: f32 },
}

#[derive(Copy, Clone, Component)]
pub struct LightFade {
    pub tt: f32,
    pub direction: f32,
    pub start_at: f32,
    pub complete_at: f32,
}

impl LightFade {
    pub fn fade_out(duration: f32) -> Self {
        Self::fade_out_delayed(0.0, duration)
    }

    pub fn fade_out_delayed(start_at: f32, duration: f32) -> Self {
        Self::delayed(-1.0, start_at, duration)
    }

    pub fn fade_in(duration: f32) -> Self {
        Self::fade_in_delayed(0.0, duration)
    }

    pub fn fade_in_delayed(start_at: f32, duration: f32) -> Self {
        Self::delayed(1.0, start_at, duration)
    }

    fn delayed(direction: f32, start_at: f32, duration: f32) -> Self {
        assert!(duration > 0.0);

        Self {
            tt: 0.0,
            direction,
            start_at,
            complete_at: start_at + duration,
        }
    }

    pub(crate) fn animate(
        time: Res<Time>,
        mut commands: Commands,
        mut lights: Query<(Entity, &mut Self, &mut Light)>,
    ) {
        for (entity, mut this, mut light) in lights.iter_mut() {
            this.tt += time.delta_seconds();

            light.intensity = if this.tt < this.start_at {
                0.0
            } else if this.tt > this.complete_at {
                1.0
            } else {
                (this.tt - this.start_at) / (this.complete_at - this.start_at)
            };

            if this.direction < 0.0 {
                light.intensity = 1.0 - light.intensity;
            }

            if this.tt > this.complete_at {
                if this.direction > 0.0 {
                    commands.entity(entity).remove::<Self>();
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
pub struct Camera {
    pub origin: Vec3,
    pub look_at: Vec3,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
pub struct Material {
    pub double_sided: Option<bool>,
    pub alpha: Option<f32>,
    pub color: Option<Color>,
    pub emissive: bool,
    pub reflectivity: Option<f32>,
    pub reflection_color: Option<Color>,
    pub texture: Option<AssetHandle<Texture>>,
    pub texture_enabled: Option<bool>,
    pub casts_shadows: Option<bool>,
    pub uv_divisor: Option<(u8, u8)>,
    pub uv_transparency: Option<bool>,
}

impl Material {
    pub fn double_sided(mut self) -> Self {
        self.double_sided = Some(true);
        self
    }

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

    pub fn with_texture(mut self, texture: AssetHandle<Texture>) -> Self {
        self.texture = Some(texture);
        self.texture_enabled = Some(true);
        self
    }

    pub fn without_texture(mut self) -> Self {
        self.texture_enabled = Some(false);
        self
    }

    pub fn without_casting_shadows(mut self) -> Self {
        self.casts_shadows = Some(false);
        self
    }

    pub fn with_uv_divisor(mut self, u: u8, v: u8) -> Self {
        self.uv_divisor = Some((u, v));
        self
    }

    pub fn with_uv_transparency(mut self) -> Self {
        self.uv_transparency = Some(true);
        self
    }

    pub fn emissive(mut self) -> Self {
        self.emissive = true;
        self
    }

    pub(crate) fn merge_with(self, other: Self) -> Self {
        Self {
            double_sided: self.double_sided.or(other.double_sided),
            alpha: self.alpha.or(other.alpha),
            color: self.color.or(other.color),
            emissive: self.emissive || other.emissive,
            reflectivity: self.reflectivity.or(other.reflectivity),
            reflection_color: self.reflection_color.or(other.reflection_color),
            texture: self.texture.or(other.texture),
            texture_enabled: self.texture_enabled.or(other.texture_enabled),
            casts_shadows: self.casts_shadows.or(other.casts_shadows),
            uv_divisor: self.uv_divisor.or(other.uv_divisor),
            uv_transparency: self.uv_transparency.or(other.uv_transparency),
        }
    }

    pub(crate) fn materialize(self) -> rt::Material {
        let color = self.color.unwrap_or_default().into_vec3();
        let texture = self.texture_enabled == Some(true);
        let reflectivity = self.reflectivity.unwrap_or_default();
        let reflection_color =
            self.reflection_color.unwrap_or_default().into_vec3();

        rt::Material::default()
            .with_color(color)
            .with_texture(texture)
            .with_reflectivity(reflectivity, reflection_color)
            .with_emissive(self.emissive)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Component)]
pub enum GeometryType {
    Static,
    Dynamic,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Component)]
pub struct Visibility {
    pub is_visible: bool,
}

impl Visibility {
    pub fn visible() -> Self {
        Self { is_visible: true }
    }

    pub fn invisible() -> Self {
        Self { is_visible: false }
    }
}

/// Marker-component determing whether the raytracer already knows of given
/// entity or not; it's used to allocate and release entity's geometry and
/// materials in the raytracer's internal data structures.
///
/// This component is added and removed by the raytracer's syncing systems and
/// shouldn't be added / removed manually.
#[derive(Component)]
pub(crate) struct Synced;
