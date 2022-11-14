use glam::{Mat4, Vec3};

pub fn identity() -> Mat4 {
    Mat4::IDENTITY
}

/// Translates the transformation matrix by the given translation vector.
pub fn translate(xform: &mut Mat4, v: Vec3) -> &mut Mat4 {
    *xform *= Mat4::from_translation(v);
    xform
}

/// Scales the transformation matrix along axes given by the vector.
pub fn scale(xform: &mut Mat4, v: Vec3) -> &mut Mat4 {
    *xform *= Mat4::from_scale(v);
    xform
}

/// Rotates the transation matrix by the given angle (in degrees) around the given axis.
pub fn rotate(xform: &mut Mat4, angle: f32, axis: Vec3) -> &mut Mat4 {
    *xform *= Mat4::from_axis_angle(axis, angle);
    xform
}

/// Applies the transformation matrix to the point.
pub fn apply_transformation(v: Vec3, xform: Mat4) -> Vec3 {
    let v = xform * v.extend(1.0);
    Vec3::new(v.x, v.y, v.z)
}
