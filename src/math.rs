pub fn remap(
    v: f32,
    min_from: f32,
    max_from: f32,
    min_to: f32,
    max_to: f32,
) -> f32 {
    let t = (v - min_from) / (max_from - min_from);
    min_to + t * (max_to - min_to)
}
