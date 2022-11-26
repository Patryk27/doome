use glam::Vec2;

use crate::Polygon;

/// Separating Axis Theorem
/// https://dyn4j.org/2010/01/sat/

/// Resolves the SAT collision between two polygons
///
/// Returns the minimum translation vector required to resolve the collision, if such occurred,
/// otherwise returns None.
pub fn resolve_sat(a: &Polygon, b: &Polygon) -> Option<Vec2> {
    let all_axes = iter_separation_axes(a).chain(iter_separation_axes(b));

    let mut min_overlap = std::f32::MAX;
    let mut min_mtv = Vec2::ZERO;

    for axis in all_axes {
        let a_proj = Proj::project(a.points(), axis);
        let b_proj = Proj::project(b.points(), axis);

        // If there's no overlap we'll early return None
        let (mtv, overlap) = resolve_axis_projections(axis, a_proj, b_proj)?;

        if overlap < min_overlap {
            min_overlap = overlap;
            min_mtv = mtv;
        }
    }

    Some(min_mtv)
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

    let mut overlap = a_overlap.min(b_overlap);

    let mut sign = 1.0;

    if a.contains(b) || b.contains(a) {
        let mins = (a.min - b.min).abs();
        let maxs = (a.max - b.max).abs();

        if mins < maxs {
            overlap += mins;
            sign = -1.0;
        } else {
            overlap += maxs;
        }
    }

    Some((axis * overlap * sign, overlap))
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

    pub fn contains(self, other: Self) -> bool {
        self.min <= other.min && self.max >= other.max
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
