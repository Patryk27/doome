use glam::Vec2;

/// Separating Axis Theorem
/// https://dyn4j.org/2010/01/sat/

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Vec2>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Self { vertices }
    }

    pub fn iter_edge_vectors(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.vertices.iter().enumerate().map(move |(i, v)| {
            let v = *v;

            let next = self.vertices[(i + 1) % self.vertices.len()];

            next - v
        })
    }

    pub fn iter_separation_axes(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.iter_edge_vectors().map(|v| v.perp().normalize())
    }

    pub fn project_vertices_onto(&self, axis: Vec2) -> Vec<Vec2> {
        self.vertices
            .iter()
            .map(|v| axis * v.dot(axis))
            .collect::<Vec<_>>()
    }
}

pub fn resolve_sat(a: &Polygon, b: &Polygon) -> bool {
    let all_axes: Vec<Vec2> = a
        .iter_separation_axes()
        .chain(b.iter_separation_axes())
        .collect();

    for axis in all_axes {
        let a_vertices = a.project_vertices_onto(axis);
        let b_vertices = b.project_vertices_onto(axis);

        let a_min = a_vertices
            .iter()
            .map(|v| v.dot(axis))
            .fold(f32::NAN, f32::min);
        let a_max = a_vertices
            .iter()
            .map(|v| v.dot(axis))
            .fold(f32::NAN, f32::max);

        let b_min = b_vertices
            .iter()
            .map(|v| v.dot(axis))
            .fold(f32::NAN, f32::min);
        let b_max = b_vertices
            .iter()
            .map(|v| v.dot(axis))
            .fold(f32::NAN, f32::max);

        if a_max < b_min || b_max < a_min {
            return false;
        }
    }

    true
}
