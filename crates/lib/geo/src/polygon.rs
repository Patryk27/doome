use std::f32::consts::PI;

use glam::Vec2;

const LINE_THICKNESS: f32 = 0.25;

/// A 2D convex polygon
#[derive(Debug, Clone)]
pub struct Polygon {
    points: Vec<Vec2>,
}

/// A convex polygon with points oriented in a counter-clockwise fashion.
///
/// The counter-clockwise arrangement is important for the SAT algorithm to work.
/// It is because we want the edge normals for collisions to point away from the polygon.
/// And the `Vec::perp` method returns a vector rotated 90 degrees clockwise.
impl Polygon {
    pub(crate) fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }

    pub fn rect(half_extents: Vec2) -> Self {
        Self::new(vec![
            Vec2::new(-half_extents.x, -half_extents.y),
            Vec2::new(half_extents.x, -half_extents.y),
            Vec2::new(half_extents.x, half_extents.y),
            Vec2::new(-half_extents.x, half_extents.y),
        ])
    }

    /// We don't actually support lines with our collision implementation, so lines are just very thin rectangles
    pub fn line(start: Vec2, end: Vec2) -> Self {
        let line_vec = end - start;
        let half_thickness =
            line_vec.perp().normalize_or_zero() * LINE_THICKNESS * 0.5;

        let start = start - half_thickness;
        let end = end + half_thickness;

        Self::rect_start_end(start, end)
    }

    pub fn rect_start_end(start: Vec2, end: Vec2) -> Self {
        Self::new(vec![
            Vec2::new(start.x, start.y),
            Vec2::new(end.x, start.y),
            Vec2::new(end.x, end.y),
            Vec2::new(start.x, end.y),
        ])
    }

    pub fn circle(radius: f32, num_points: usize) -> Self {
        let mut points = Vec::with_capacity(num_points);
        let angle = 2.0 * PI / num_points as f32;
        for i in 0..num_points {
            let angle = angle * i as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            points.push(Vec2::new(x, y));
        }
        Self { points }
    }

    pub fn offset(mut self, offset: Vec2) -> Self {
        for point in self.points.iter_mut() {
            *point += offset;
        }

        self
    }

    pub fn points(&self) -> &[Vec2] {
        &self.points
    }

    /// Iterates over "edge vectors" i.e. 2d vectors derived from edges.
    pub fn iter_edge_vectors(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.iter_edges().map(|(a, b)| b - a)
    }

    /// Iterates over edges of this polygon
    pub fn iter_edges(&self) -> impl Iterator<Item = (Vec2, Vec2)> + '_ {
        self.points.iter().enumerate().map(move |(i, v)| {
            let v = *v;

            let next = self.points[(i + 1) % self.points.len()];

            (v, next)
        })
    }

    pub fn num_edges(&self) -> usize {
        self.points.len()
    }

    pub fn map_points<F>(mut self, m: F) -> Self
    where
        F: Fn(Vec2) -> Vec2,
    {
        for point in self.points.iter_mut() {
            *point = m(*point);
        }
        self
    }
}
