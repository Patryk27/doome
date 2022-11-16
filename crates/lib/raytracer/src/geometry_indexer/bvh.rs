use super::*;

#[derive(Clone)]
pub struct Bvh {
    root: BvhNode,
}

impl Bvh {
    pub fn build(geometry: &StaticGeometry) -> Self {
        let mut root = BvhNode::default();

        for (triangle_id, triangle) in geometry.iter() {
            root.add(triangle_id, triangle);
        }

        root.balance();

        Self { root }
    }

    pub fn into_root(self) -> BvhNode {
        self.root
    }
}

#[derive(Clone, Default)]
pub struct BvhNode {
    bb: BoundingBox,
    triangles: Vec<(TriangleId, Triangle)>,
    children: Option<[Box<Self>; 2]>,
}

impl BvhNode {
    fn add(&mut self, triangle_id: TriangleId, triangle: Triangle) {
        for vertex in triangle.vertices() {
            self.bb.grow(vertex);
        }

        self.triangles.push((triangle_id, triangle));
    }

    fn balance(&mut self) {
        let mut best = None;

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            for (_, triangle) in &self.triangles {
                let cost = self.estimate_balancing_by(axis, triangle.center());

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
                self.balance_by(axis, triangle.center());
            }
        }
    }

    fn estimate_balancing_by(&self, axis: Axis, pos: Vec3) -> f32 {
        let mut left = 0;
        let mut left_bb = BoundingBox::default();
        let mut right = 0;
        let mut right_bb = BoundingBox::default();

        for (_, triangle) in &self.triangles {
            let (side, side_bb) = if triangle.center()[axis] < pos[axis] {
                (&mut left, &mut left_bb)
            } else {
                (&mut right, &mut right_bb)
            };

            *side += 1;

            for vertex in triangle.vertices() {
                side_bb.grow(vertex);
            }
        }

        let cost =
            (left as f32) * left_bb.area() + (right as f32) * right_bb.area();

        cost.max(1.0)
    }

    fn balance_by(&mut self, axis: Axis, pos: Vec3) {
        let mut left = Self::default();
        let mut right = Self::default();

        for (triangle_id, triangle) in self.triangles.drain(..) {
            let side = if triangle.center()[axis] < pos[axis] {
                &mut left
            } else {
                &mut right
            };

            side.add(triangle_id, triangle);
        }

        left.balance();
        right.balance();

        self.children = Some([Box::new(left), Box::new(right)]);
    }

    pub fn deconstruct(self) -> DeconstructedBvhNode {
        if let Some([left, right]) = self.children {
            DeconstructedBvhNode::NonLeaf {
                bb: self.bb,
                left: Box::new(left.deconstruct()),
                right: Box::new(right.deconstruct()),
            }
        } else {
            DeconstructedBvhNode::Leaf {
                triangles: self
                    .triangles
                    .into_iter()
                    .map(|(id, _)| id)
                    .collect(),
            }
        }
    }
}

impl fmt::Display for BvhNode {
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

            for (triangle_idx, (triangle_id, _)) in
                self.triangles.iter().enumerate()
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

pub enum DeconstructedBvhNode {
    Leaf {
        triangles: Vec<TriangleId>,
    },

    NonLeaf {
        bb: BoundingBox,
        left: Box<Self>,
        right: Box<Self>,
    },
}
