use glam::{Vec2, Vec4};
use spirv_std::{Image, Sampler};

pub struct Texture<'a> {
    pub texture: &'a Image!(2D, type=f32, sampled),
    pub sampler: &'a Sampler,
}

impl<'a> Texture<'a> {
    pub fn sample(&self, uv: Vec2) -> Vec4 {
        self.texture.sample(*self.sampler, uv)
    }
}
