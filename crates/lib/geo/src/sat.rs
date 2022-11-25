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
        let a_vertices = project_vertices_onto(a.points(), axis);
        let b_vertices = project_vertices_onto(b.points(), axis);

        // If there's no overlap we'll early return None
        let (mtv, overlap) =
            resolve_axis_projections(axis, &a_vertices, &b_vertices)?;

        if overlap < min_overlap {
            min_overlap = overlap;
            min_mtv = mtv;
        }
    }

    Some(min_mtv)
}

fn iter_separation_axes(poly: &Polygon) -> impl Iterator<Item = Vec2> + '_ {
    poly.iter_edge_vectors()
        .flat_map(|v| [v.perp().normalize(), -v.perp().normalize()])
}

/// Resolves axis projections
///
/// if the projections overlap, returns the axis multiplied by the overlap sector
/// otherwise returns None
fn resolve_axis_projections(
    axis: Vec2,
    a: &[f32],
    b: &[f32],
) -> Option<(Vec2, f32)> {
    let a_min = a.iter().copied().fold(f32::NAN, f32::min);
    let a_max = a.iter().copied().fold(f32::NAN, f32::max);
    let b_min = b.iter().copied().fold(f32::NAN, f32::min);
    let b_max = b.iter().copied().fold(f32::NAN, f32::max);

    if a_max < b_min || b_max < a_min {
        return None;
    }

    let a_overlap = a_max - b_min;
    let b_overlap = b_max - a_min;
    let mut overlap = a_overlap.min(b_overlap);
    let mut sign = 1.0;

    if (a_min >= b_min && a_max <= b_max) || (b_min >= a_min && b_max <= a_max)
    {
        let mins = (a_min - b_min).abs();
        let maxs = (a_max - b_max).abs();

        if mins < maxs {
            overlap += mins;
            sign *= -1.0;
        } else {
            overlap += maxs;
        }
    }

    Some((axis * overlap * sign, overlap))
}

fn project_vertices_onto(vertices: &[Vec2], axis: Vec2) -> Vec<f32> {
    vertices.iter().map(|v| axis.dot(*v)).collect::<Vec<_>>()
}
