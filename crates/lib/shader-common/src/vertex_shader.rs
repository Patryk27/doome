use glam::{vec2, Vec2, Vec4};

/// Creates a "full screen triangle" by mapping the vertex index.
/// ported from https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
pub fn full_screen_triangle(vert_idx: i32) -> Vec4 {
    let uv = vec2(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    pos.extend(0.0).extend(1.0)
}
