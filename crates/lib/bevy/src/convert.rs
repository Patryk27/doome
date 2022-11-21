use glam::{Vec2, Vec3, Vec3Swizzles};

pub fn graphical_to_physical(v: Vec3) -> Vec2 {
    v.xz()
}

pub fn physical_to_graphical(v: Vec2) -> Vec3 {
    v.extend(0.0).xzy()
}
