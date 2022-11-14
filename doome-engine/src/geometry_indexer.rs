use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};

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

        root.balance();

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
    bb: BoundingBox,
    triangles: Vec<TriangleId>,
    children: Option<[Box<Self>; 2]>,
}

impl<'a> Tree<'a> {
    fn new(lookup: &'a HashMap<TriangleId, Triangle>) -> Self {
        Self {
            lookup,
            bb: Default::default(),
            triangles: Default::default(),
            children: Default::default(),
        }
    }

    fn add(&mut self, triangle_id: TriangleId) {
        self.triangles.push(triangle_id);

        for vertex in self.lookup[&triangle_id].vertices() {
            self.bb.grow(vertex);
        }
    }

    fn balance(&mut self) {
        let mut best = None;

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            for triangle in &self.triangles {
                let cost = self.estimate_balancing_by(
                    axis,
                    self.lookup[triangle].center(),
                );

                if let Some((best_cost, best_axis, best_triangle)) = &mut best {
                    if cost < *best_cost {
                        *best_cost = cost;
                        *best_axis = axis;
                        *best_triangle = triangle;
                    }
                } else {
                    best = Some((cost, axis, triangle));
                }
            }
        }

        if let Some((cost, axis, triangle)) = best {
            let curr_cost = (self.triangles.len() as f32) * self.bb.area();

            if cost < curr_cost {
                self.balance_by(axis, self.lookup[triangle].center());
            }
        }
    }

    fn estimate_balancing_by(&self, axis: Axis, pos: Vec3) -> f32 {
        let mut left = 0;
        let mut left_bb = BoundingBox::default();
        let mut right = 0;
        let mut right_bb = BoundingBox::default();

        for triangle in &self.triangles {
            let (side, side_bb) =
                if self.lookup[triangle].center()[axis] < pos[axis] {
                    (&mut left, &mut left_bb)
                } else {
                    (&mut right, &mut right_bb)
                };

            *side += 1;

            for vertex in self.lookup[triangle].vertices() {
                side_bb.grow(vertex);
            }
        }

        // TODO the hard-coded 100.0 here feels kinda arbitrary
        let cost = 100.0
            + (left as f32) * left_bb.area()
            + (right as f32) * right_bb.area();

        cost.max(1.0)
    }

    fn balance_by(&mut self, axis: Axis, pos: Vec3) {
        let mut left = Self::new(self.lookup);
        let mut right = Self::new(self.lookup);

        for triangle_id in self.triangles.drain(..) {
            if self.lookup[&triangle_id].center()[axis] < pos[axis] {
                left.add(triangle_id);
            } else {
                right.add(triangle_id);
            }
        }

        left.balance();
        right.balance();

        self.children = Some([Box::new(left), Box::new(right)]);
    }

    fn serialize(&self, root: bool) -> Vec<Vec4> {
        let mut out = Vec::new();

        if root {
            out.push(vec4(1.0, 2.0, 3.0, 4.0));
        }

        if let Some(children) = &self.children {
            let left = children[0].serialize(false);
            let right = children[1].serialize(false);

            assert!(left.len() > 0);

            out.push(self.bb.min().extend(left.len() as _));
            out.push(self.bb.max().extend(0.0));
            out.extend(left);
            out.extend(right);
        } else {
            assert!(self.triangles.len() > 0);

            out.push(self.bb.min().extend(0.0));
            out.push(self.bb.max().extend(self.triangles.len() as _));

            for mut triangles in self.triangles.chunks(4) {
                let mut vec = vec4(0.0, 0.0, 0.0, 0.0);

                if !triangles.is_empty() {
                    vec.x = triangles[0] as _;
                    triangles = &triangles[1..];
                }

                if !triangles.is_empty() {
                    vec.y = triangles[0] as _;
                    triangles = &triangles[1..];
                }

                if !triangles.is_empty() {
                    vec.z = triangles[0] as _;
                    triangles = &triangles[1..];
                }

                if !triangles.is_empty() {
                    vec.w = triangles[0] as _;
                }

                out.push(vec);
            }
        }

        out
    }
}

impl fmt::Display for Tree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} .. {}", self.bb.min(), self.bb.max())?;

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

#[derive(Clone, Copy, Debug, Default)]
struct BoundingBox {
    min: Option<Vec3>,
    max: Option<Vec3>,
}

impl BoundingBox {
    fn grow(&mut self, p: Vec3) {
        if let Some(min) = &mut self.min {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
        } else {
            self.min = Some(p);
        }

        if let Some(max) = &mut self.max {
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        } else {
            self.max = Some(p);
        }
    }

    fn min(&self) -> Vec3 {
        self.min.unwrap()
    }

    fn max(&self) -> Vec3 {
        self.max.unwrap()
    }

    fn area(&self) -> f32 {
        if let (Some(min), Some(max)) = (self.min, self.max) {
            let extent = max - min;

            assert!(extent.length() > 0.0);

            extent.x * extent.y + extent.y * extent.z + extent.z * extent.x
        } else {
            0.0
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

impl Index<Axis> for Vec3 {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl IndexMut<Axis> for Vec3 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}
