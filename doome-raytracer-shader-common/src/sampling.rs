use crate::*;

pub struct Texture<'a> {
    pub texture: &'a Image!(2D, type=f32, sampled),
    pub sampler: &'a Sampler,
}

impl<'a> Texture<'a> {
    pub fn sample(&self, uv: Vec2) -> Vec4 {
        self.texture.sample_by_lod(*self.sampler, uv, 0.0)
    }
}
