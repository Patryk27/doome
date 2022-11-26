use glam::Vec2;

use crate::Polygon;

/// Separating Axis Theorem
/// https://dyn4j.org/2010/01/sat/

/// Resolves the SAT collision between two polygons
///
/// Returns the minimum translation vector required to resolve the collision, if such occurred,
/// otherwise returns None.
///
/// The returned vector should be such that adding it to the first polygon will move it out of the collision
pub fn resolve_sat(a: &Polygon, b: &Polygon) -> Option<Vec2> {
    let a_axes = iter_separation_axes(a);
    let b_axes = iter_separation_axes(b);

    let mut min_overlap = std::f32::MAX;
    let mut min_mtv = Vec2::ZERO;

    for axis in a_axes {
        let a_proj = Proj::project(a.points(), axis);
        let b_proj = Proj::project(b.points(), axis);

        // If there's no overlap we'll early return None
        let (mtv, overlap) = resolve_axis_projections(axis, a_proj, b_proj)?;

        if overlap < min_overlap {
            min_overlap = overlap;
            min_mtv = mtv;
        }
    }

    for axis in b_axes {
        let a_proj = Proj::project(a.points(), axis);
        let b_proj = Proj::project(b.points(), axis);

        // If there's no overlap we'll early return None
        let (mtv, overlap) = resolve_axis_projections(axis, a_proj, b_proj)?;

        if overlap < min_overlap {
            min_overlap = overlap;
            min_mtv = mtv;
        }
    }

    // We return a negated mtv so that we can add it to the first polygon
    Some(-min_mtv)
}

fn iter_separation_axes(poly: &Polygon) -> impl Iterator<Item = Vec2> + '_ {
    poly.iter_edge_vectors().map(|v| v.perp().normalize())
}

/// Resolves axis projections
///
/// if the projections overlap, returns the axis multiplied by the overlap sector
/// otherwise returns None
fn resolve_axis_projections(
    axis: Vec2,
    a: Proj,
    b: Proj,
) -> Option<(Vec2, f32)> {
    if !a.is_overlapping(b) {
        return None;
    }

    let a_overlap = a.overlap(b);
    let b_overlap = b.overlap(a);

    let overlap = a_overlap.min(b_overlap);

    // TODO: Introduce this back at some point to support lines and containing collisions
    // let mut sign = 1.0;

    // if a.contains(b) || b.contains(a) {
    //     let mins = (a.min - b.min).abs();
    //     let maxs = (a.max - b.max).abs();

    //     if mins < maxs {
    //         overlap += mins;
    //         sign = -1.0;
    //     } else {
    //         overlap += maxs;
    //     }
    // }

    Some((axis * overlap, overlap))
}

/// Projection of polygon points onto an axis
#[derive(Clone, Copy)]
struct Proj {
    min: f32,
    max: f32,
}

impl Proj {
    pub fn project(points: &[Vec2], axis: Vec2) -> Self {
        let mut min = project_vertex(points[0], axis);
        let mut max = min;

        for point in points.iter().skip(1) {
            let proj = project_vertex(*point, axis);

            min = min.min(proj);
            max = max.max(proj);
        }

        Self { min, max }
    }

    pub fn is_overlapping(self, other: Self) -> bool {
        self.max >= other.min && other.max >= self.min
    }

    pub fn overlap(self, other: Self) -> f32 {
        self.max - other.min
    }
}

fn project_vertex(v: Vec2, axis: Vec2) -> f32 {
    v.dot(axis)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use glam::vec2;

    use super::*;

    const PRECISION: f32 = 0.0001;

    fn simf(a: f32, b: f32) -> bool {
        (a - b).abs() < PRECISION
    }

    fn sim(a: &Vec2, b: &Vec2) -> bool {
        simf(a.x, b.x) && simf(a.y, b.y)
    }

    fn assert_sim(a: &Vec2, b: &Vec2) {
        assert!(sim(a, b), "Vectors {a:?} and {b:?} should be simmilar");
    }

    #[test]
    fn basic_rects() {
        let rect_a = Polygon::rect(vec2(1.0, 1.0)).offset(vec2(-0.5, 0.0));
        let rect_b = Polygon::rect(vec2(1.0, 1.0)).offset(vec2(0.5, 0.0));

        println!("{rect_a:?}");
        println!("{rect_b:?}");

        let mtv = resolve_sat(&rect_a, &rect_b).unwrap();

        assert_sim(&mtv, &vec2(1.0, 0.0));
    }

    #[test]
    fn perp_lines() {
        let rect_a = Polygon::new(vec![vec2(-1.0, 0.0), vec2(1.0, 0.0)]);
        let rect_b = Polygon::new(vec![vec2(0.0, -1.0), vec2(0.0, 1.0)]);

        let mtv = resolve_sat(&rect_a, &rect_b).unwrap();

        assert_sim(&mtv, &vec2(0.0, -1.0));
    }

    #[test]
    fn triangle_to_line_from_left() {
        let line = Polygon::new(vec![vec2(0.0, -1.0), vec2(0.0, 1.0)]);

        let triangle = Polygon::new(vec![
            vec2(0.2, 0.0), // the intersecting vertex, it penetrates the line by 0.2 pointing to the right
            vec2(-1.0, 1.0),
            vec2(-1.0, -1.0),
        ]);

        let mtv = resolve_sat(&triangle, &line).unwrap();

        assert_sim(&mtv, &vec2(-0.2, 0.0));
    }

    #[test]
    fn triangle_to_line_from_right() {
        let line = Polygon::new(vec![vec2(0.0, -1.0), vec2(0.0, 1.0)]);

        let triangle = Polygon::new(vec![
            vec2(-0.2, 0.0), // the intersecting vertex, it penetrates the line by 0.2 pointing to the left
            vec2(1.0, -1.0),
            vec2(1.0, 1.0),
        ]);

        let mtv = resolve_sat(&triangle, &line).unwrap();

        assert_sim(&mtv, &vec2(0.2, 0.0));
    }
}
