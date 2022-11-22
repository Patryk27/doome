use glam::Vec2;

// https://stackoverflow.com/a/565282
pub fn intersect(p: Vec2, r: Vec2, q: Vec2, s: Vec2) -> Option<Vec2> {
    let rs = cross(r, s);

    let qp = q - p;
    let qp_cross_r = cross(qp, r);
    let t = cross(qp, s) / rs;
    let u = cross(qp, r) / rs;

    if rs == 0.0 && qp_cross_r == 0.0 {
        // The 2 vectors are collinear
        // TODO: This could lead to some weird edge cases
        // but I don't think it makes sense to return anyhing here
        None
    } else if rs == 0.0 && qp_cross_r != 0.0 {
        // The lines are parallel
        None
    } else if rs != 0.0 && (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)
    {
        // The vectors intersect
        Some(p + r * t)
    } else {
        // Lines intersect, but vectors do not
        None
    }
}

fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

#[cfg(test)]
mod test {
    use glam::vec2;

    use super::*;
    #[test]
    fn perp_intersect_at_zero() {
        let p = vec2(-1.0, -1.0);
        let r = vec2(1.0, 1.0);

        let q = vec2(1.0, -1.0);
        let s = vec2(-1.0, 1.0);

        assert_eq!(intersect(p, r, q, s), Some(vec2(0.0, 0.0)));
    }
}
