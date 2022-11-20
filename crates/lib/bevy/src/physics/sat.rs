use doome_geo::Polygon;
use glam::Vec2;

/// Separating Axis Theorem
/// https://dyn4j.org/2010/01/sat/

/// Resolves the SAT collision between two polygons
///
/// Returns the minimum translation vector required to resolve the collision, if such occurred,
/// otherwise returns None.
pub fn resolve_sat(a: &Polygon, b: &Polygon) -> Option<Vec2> {
    let all_axes: Vec<Vec2> = iter_separation_axes(a)
        .chain(iter_separation_axes(b))
        .collect();

    let mut mtvs = Vec::with_capacity(all_axes.len());
    for axis in all_axes {
        let a_vertices = project_vertices_onto(a.points(), axis);
        let b_vertices = project_vertices_onto(b.points(), axis);

        // If there's no overlap we'll early return None
        mtvs.push(resolve_axis_projections(axis, &a_vertices, &b_vertices)?);
    }

    mtvs.into_iter().min_by(|a, b| {
        a.length()
            .partial_cmp(&b.length())
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

fn iter_separation_axes(poly: &Polygon) -> impl Iterator<Item = Vec2> + '_ {
    poly.iter_edge_vectors().map(|v| v.perp().normalize())
}

/// Resolves axis projections
///
/// if the projections overlap, returns the axis multiplied by the overlap sector
/// otherwise returns None
pub fn resolve_axis_projections(
    axis: Vec2,
    a: &[Vec2],
    b: &[Vec2],
) -> Option<Vec2> {
    let a_min = a.iter().map(|v| v.dot(axis)).fold(f32::NAN, f32::min);
    let a_max = a.iter().map(|v| v.dot(axis)).fold(f32::NAN, f32::max);

    let b_min = b.iter().map(|v| v.dot(axis)).fold(f32::NAN, f32::min);
    let b_max = b.iter().map(|v| v.dot(axis)).fold(f32::NAN, f32::max);

    if a_max < b_min || b_max < a_min {
        None
    } else {
        let a_overlap = a_max - b_min;
        let b_overlap = b_max - a_min;

        let overlap = a_overlap.min(b_overlap);

        Some(axis * overlap)
    }
}

pub fn project_vertices_onto(vertices: &[Vec2], axis: Vec2) -> Vec<Vec2> {
    vertices
        .iter()
        .map(|v| axis * v.dot(axis))
        .collect::<Vec<_>>()
}
