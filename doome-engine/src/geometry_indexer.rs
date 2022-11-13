use std::collections::HashMap;
use std::fmt;

use doome_raytracer_shader_common::{GeometryIndex, Triangle};
use glam::{vec4, Vec3, Vec4};

type TriangleId = u16;

#[derive(Default)]
pub struct GeometryIndexer {
    triangles: HashMap<TriangleId, Triangle>,
}

impl GeometryIndexer {
    pub fn add(&mut self, triangle: Triangle, triangle_id: TriangleId) {
        self.triangles.insert(triangle_id, triangle);
    }

    pub fn build(self) -> GeometryIndex {
        let mut root = Tree::new(&self.triangles);

        for &triangle_id in self.triangles.keys() {
            root.add(triangle_id);
        }

        root.balance(0);

        // ---

        log::info!("BVH:\n{}", root);

        // ---

        let mut data = root.serialize(true);

        while data.len() < 2048 {
            data.push(vec4(0.0, 0.0, 0.0, 0.0));
        }

        let data = data.try_into().unwrap_or_else(|data: Vec<_>| {
            panic!(
                "ayy ayy the geometry index is too large -- produced {} bytes",
                data.len()
            );
        });

        GeometryIndex::new(data)
    }
}

struct Tree<'a> {
    lookup: &'a HashMap<TriangleId, Triangle>,
    bb_min: Option<Vec3>,
    bb_max: Option<Vec3>,
    triangles: Vec<TriangleId>,
    children: Option<[Box<Self>; 2]>,
}

impl<'a> Tree<'a> {
    fn new(lookup: &'a HashMap<TriangleId, Triangle>) -> Self {
        Self {
            lookup,
            bb_min: Default::default(),
            bb_max: Default::default(),
            triangles: Default::default(),
            children: Default::default(),
        }
    }

    fn add(&mut self, triangle_id: TriangleId) {
        for vertex in self.lookup[&triangle_id].vertices() {
            if let Some(bb_min) = &mut self.bb_min {
                bb_min.x = bb_min.x.min(vertex.x);
                bb_min.y = bb_min.y.min(vertex.y);
                bb_min.z = bb_min.z.min(vertex.z);
            } else {
                self.bb_min = Some(vertex);
            }

            if let Some(bb_max) = &mut self.bb_max {
                bb_max.x = bb_max.x.max(vertex.x);
                bb_max.y = bb_max.y.max(vertex.y);
                bb_max.z = bb_max.z.max(vertex.z);
            } else {
                self.bb_max = Some(vertex);
            }
        }

        self.triangles.push(triangle_id);
    }

    fn balance(&mut self, depth: usize) {
        if self.triangles.len() < 32 || depth > 16 {
            return;
        }

        let (mid_x, score_x) = self.estimate_splitting_by(|p| p.x);
        let (mid_y, score_y) = self.estimate_splitting_by(|p| p.y);
        let (mid_z, score_z) = self.estimate_splitting_by(|p| p.z);

        #[derive(Clone, Copy, Debug)]
        enum SplitBy {
            X,
            Y,
            Z,
        }

        let split_by = if score_x < score_y && score_x < score_z {
            SplitBy::X
        } else if score_y < score_x && score_y < score_z {
            SplitBy::Y
        } else if score_z < score_x && score_z < score_y {
            SplitBy::Z
        } else {
            return;
        };

        match split_by {
            SplitBy::X => self.split_by(depth, mid_x, |p| p.x),
            SplitBy::Y => self.split_by(depth, mid_y, |p| p.y),
            SplitBy::Z => self.split_by(depth, mid_z, |p| p.z),
        }
    }

    fn estimate_splitting_by(&self, f: fn(Vec3) -> f32) -> (f32, usize) {
        let sum: f32 = self
            .triangles
            .iter()
            .map(|triangle_id| f(self.lookup[triangle_id].center()))
            .sum();

        let mid = sum / (self.triangles.len() as f32);

        let mut left = 0;
        let mut right = 0;

        for triangle_id in &self.triangles {
            if f(self.lookup[triangle_id].center()) <= mid {
                left += 1;
            } else {
                right += 1;
            }
        }

        (mid, left.max(right) - left.min(right))
    }

    fn split_by(&mut self, depth: usize, mid: f32, f: fn(Vec3) -> f32) {
        let mut left = Self::new(self.lookup);
        let mut right = Self::new(self.lookup);

        for triangle_id in self.triangles.drain(..) {
            if f(self.lookup[&triangle_id].center()) <= mid {
                left.add(triangle_id);
            } else {
                right.add(triangle_id);
            }
        }

        left.balance(depth + 1);
        right.balance(depth + 1);

        self.children = Some([Box::new(left), Box::new(right)]);
    }

    fn serialize(&self, root: bool) -> Vec<Vec4> {
        let mut out = Vec::new();

        if root {
            out.push(vec4(1.0, 2.0, 3.0, 4.0));
        }

        let (bb_min, bb_max) = (self.bb_min.unwrap(), self.bb_max.unwrap());

        if let Some(children) = &self.children {
            let left = children[0].serialize(false);
            let right = children[1].serialize(false);

            assert!(left.len() > 0);

            out.push(bb_min.extend(left.len() as _));
            out.push(bb_max.extend(0.0));
            out.extend(left);
            out.extend(right);
        } else {
            assert!(self.triangles.len() > 0);

            out.push(bb_min.extend(0.0));
            out.push(bb_max.extend(self.triangles.len() as _));

            for triangle_id in &self.triangles {
                out.push(vec4(*triangle_id as _, 0.0, 0.0, 0.0));
            }
        }

        out
    }
}

impl fmt::Display for Tree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} .. {}", self.bb_min.unwrap(), self.bb_max.unwrap())?;

        if let Some(children) = &self.children {
            writeln!(f)?;

            for (child_idx, child) in children.iter().enumerate() {
                if child_idx > 0 {
                    writeln!(f, "+")?;
                }

                for line in child.to_string().trim().lines() {
                    writeln!(f, "| {}", line)?;
                }
            }
        } else {
            write!(f, ", {} triangles: ", self.triangles.len())?;

            for (triangle_idx, triangle_id) in self.triangles.iter().enumerate()
            {
                if triangle_idx > 0 {
                    write!(f, ", ")?;
                }

                if triangle_idx > 5 {
                    write!(f, "...")?;
                    break;
                }

                write!(f, "{}", triangle_id)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
