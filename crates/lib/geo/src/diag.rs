use glam::Vec2;

use crate::intersect::intersect;
use crate::Polygon;

/// https://youtu.be/7Ik2vowGcU0?t=1434
///
/// Collision resolution method using diagonals
///
/// It's actually very simple, we simply take diagonals from an arbitrarily selected "middle point" of a given polygon
/// and check for intersection with edges of the other polygon.
pub fn resolve_diag(a: &Polygon, b: &Polygon) -> Option<Vec2> {
    let a_middle = a.avg();
    let b_middle = b.avg();

    let mut displacement = Vec2::ZERO;

    for point in a.points() {
        for edge in b.iter_edges() {
            if let Some(intersection) =
                intersect(a_middle, *point - a_middle, edge.0, edge.1 - edge.0)
            {
                let d = intersection - *point;
                displacement += d;
            }
        }
    }

    // Resolving against lines in the other polygon is weird
    if b.points().len() > 2 {
        for point in b.points() {
            for edge in a.iter_edges() {
                if let Some(intersection) = intersect(
                    b_middle,
                    *point - b_middle,
                    edge.0,
                    edge.1 - edge.0,
                ) {
                    let d = intersection - *point;
                    displacement -= d;
                }
            }
        }
    }

    if displacement.x == 0.0 && displacement.y == 0.0 {
        None
    } else {
        Some(displacement)
    }
}
