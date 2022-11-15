use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};

use doome_raytracer::{GeometryIndex, Triangle};
use glam::{vec4, Vec3, Vec4};

type TriangleId = u16;

/// An BVH + LBVH geometry indexer.
///
/// Thanks to https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/.
#[derive(Default)]
pub struct GeometryIndexer {
    triangles: HashMap<TriangleId, Triangle>,
}

impl GeometryIndexer {
    pub fn add(&mut self, triangle: Triangle, triangle_id: TriangleId) {
        self.triangles.insert(triangle_id, triangle);
    }

    pub fn build(self) -> GeometryIndex {
        log::trace!("Building BVH");

        let mut bvh = BvhNode::new(&self.triangles);

        for &triangle_id in self.triangles.keys() {
            bvh.add(triangle_id);
        }

        log::info!("Balancing BVH");
        bvh.balance();
        log::trace!("... BVH:\n{}", bvh);

        // -----

        log::info!("Linearizing BVH");
        let lbvh = bvh.linearize();
        log::trace!("... LBVH:\n{}", lbvh);

        // -----

        let mut data = lbvh.serialize();

        while data.len() < 4096 {
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

struct BvhNode<'a> {
    lookup: &'a HashMap<TriangleId, Triangle>,
    bb: BoundingBox,
    triangles: Vec<TriangleId>,
    children: Option<[Box<Self>; 2]>,
}

impl<'a> BvhNode<'a> {
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

        let cost =
            (left as f32) * left_bb.area() + (right as f32) * right_bb.area();

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

    fn linearize(self) -> LinearBvh {
        let mut lbvh = LinearBvh::default();

        self.linearize_ex(&mut lbvh, None);

        lbvh
    }

    fn linearize_ex(
        self,
        lbvh: &mut LinearBvh,
        backtrack_to: Option<usize>,
    ) -> usize {
        if let Some([left, right]) = self.children {
            let id = lbvh.add_non_leaf(self.bb);
            let right_id = right.linearize_ex(lbvh, backtrack_to);
            let left_id = left.linearize_ex(lbvh, Some(right_id));

            lbvh.fixup_non_leaf(id, left_id, backtrack_to);

            id
        } else {
            let mut id = None;
            let mut prev_node_id = None;

            for triangle in self.triangles {
                let node_id = lbvh.add_leaf(triangle);

                if id.is_none() {
                    id = Some(node_id);
                }

                if let Some(prev_node_id) = prev_node_id {
                    lbvh.fixup_leaf(prev_node_id, node_id);
                }

                prev_node_id = Some(node_id);
            }

            if let Some(backtrack_to) = backtrack_to {
                lbvh.fixup_leaf(prev_node_id.unwrap(), backtrack_to);
            }

            id.unwrap()
        }
    }
}

impl fmt::Display for BvhNode<'_> {
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

// -----

#[derive(Default)]
struct LinearBvh {
    nodes: Vec<LinearBvhNode>,
}

impl LinearBvh {
    fn add_leaf(&mut self, triangle: TriangleId) -> usize {
        let id = self.nodes.len();

        self.nodes.push(LinearBvhNode::Leaf {
            triangle,
            goto_id: None,
        });

        id
    }

    fn fixup_leaf(&mut self, id: usize, goto_id_val: usize) {
        match &mut self.nodes[id] {
            LinearBvhNode::Leaf { goto_id, .. } => {
                *goto_id = Some(goto_id_val);
            }
            _ => unreachable!(),
        }
    }

    fn add_non_leaf(&mut self, bb: BoundingBox) -> usize {
        let id = self.nodes.len();

        self.nodes.push(LinearBvhNode::NonLeaf {
            bb,
            on_hit_goto_id: None,
            on_miss_goto_id: None,
        });

        id
    }

    fn fixup_non_leaf(
        &mut self,
        id: usize,
        on_hit_goto_id_val: usize,
        on_miss_goto_it_val: Option<usize>,
    ) {
        match &mut self.nodes[id] {
            LinearBvhNode::NonLeaf {
                on_hit_goto_id,
                on_miss_goto_id,
                ..
            } => {
                *on_hit_goto_id = Some(on_hit_goto_id_val);
                *on_miss_goto_id = on_miss_goto_it_val;
            }
            _ => unreachable!(),
        }
    }

    fn serialize(self) -> Vec<Vec4> {
        let mut out = Vec::new();

        for node in self.nodes {
            let v1;
            let v2;

            match node {
                LinearBvhNode::Leaf { triangle, goto_id } => {
                    let goto_ptr = goto_id.map(|id| id * 2).unwrap_or_default();

                    v1 = vec4(0.0, 0.0, 0.0, triangle as _);
                    v2 = vec4(0.0, 0.0, 0.0, goto_ptr as _);
                }

                LinearBvhNode::NonLeaf {
                    bb,
                    on_hit_goto_id,
                    on_miss_goto_id,
                } => {
                    let on_hit_goto_ptr =
                        on_hit_goto_id.map(|id| id * 2).unwrap_or_default();

                    let on_miss_goto_ptr =
                        on_miss_goto_id.map(|id| id * 2).unwrap_or_default();

                    v1 = bb.min().extend(on_hit_goto_ptr as _);
                    v2 = bb.max().extend(on_miss_goto_ptr as _);
                }
            }

            out.push(v1);
            out.push(v2);
        }

        out
    }
}

impl fmt::Display for LinearBvh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (node_id, node) in self.nodes.iter().enumerate() {
            writeln!(f, "[{}]: {}", node_id, node)?;
        }

        Ok(())
    }
}

enum LinearBvhNode {
    Leaf {
        triangle: TriangleId,
        goto_id: Option<usize>,
    },

    NonLeaf {
        bb: BoundingBox,
        on_hit_goto_id: Option<usize>,
        on_miss_goto_id: Option<usize>,
    },
}

impl fmt::Display for LinearBvhNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinearBvhNode::Leaf { triangle, goto_id } => {
                write!(f, "match-triangle {}", triangle)?;

                if let Some(id) = goto_id {
                    write!(f, ", goto {}", id)?;
                }
            }

            LinearBvhNode::NonLeaf {
                bb,
                on_hit_goto_id,
                on_miss_goto_id,
            } => {
                write!(f, "match-aabb {}..{}", bb.min(), bb.max())?;

                if let Some(id) = on_hit_goto_id {
                    write!(f, ", on-hit-goto {}", id)?;
                }

                if let Some(id) = on_miss_goto_id {
                    write!(f, ", on-miss-goto {}", id)?;
                }
            }
        }

        Ok(())
    }
}

// -----

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
