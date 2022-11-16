//! Linearized / Flattened BVH.
//!
//! Note that I've basically implemented the algorithm from scratch, merely
//! imagining how it should work, so the naming nomenclature might be a bit off
//! as compared to other implementations / whitepapers.

use super::*;

#[derive(Default)]
pub struct LinearBvh {
    nodes: Vec<LinearBvhNode>,
}

impl LinearBvh {
    pub fn build(bvh: Bvh) -> Self {
        let mut this = Self::default();

        this.linearize(bvh.into_root().deconstruct(), None);
        this
    }

    fn linearize(
        &mut self,
        bvh: DeconstructedBvhNode,
        backtrack_to: Option<usize>,
    ) -> usize {
        match bvh {
            DeconstructedBvhNode::Leaf { triangles } => {
                let mut id = None;
                let mut prev_node_id = None;

                for triangle in triangles {
                    let node_id = self.add_leaf(triangle);

                    if id.is_none() {
                        id = Some(node_id);
                    }

                    if let Some(prev_node_id) = prev_node_id {
                        self.fixup_leaf(prev_node_id, node_id);
                    }

                    prev_node_id = Some(node_id);
                }

                if let Some(backtrack_to) = backtrack_to {
                    self.fixup_leaf(prev_node_id.unwrap(), backtrack_to);
                }

                id.unwrap()
            }

            DeconstructedBvhNode::NonLeaf { bb, left, right } => {
                let id = self.add_non_leaf(bb);
                let right_id = self.linearize(*right, backtrack_to);
                let left_id = self.linearize(*left, Some(right_id));

                self.fixup_non_leaf(id, left_id, backtrack_to);

                id
            }
        }
    }

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
}

impl fmt::Display for LinearBvh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (node_id, node) in self.nodes.iter().enumerate() {
            writeln!(f, "[{}]: {}", node_id, node)?;
        }

        Ok(())
    }
}

impl IntoIterator for LinearBvh {
    type Item = LinearBvhNode;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

pub enum LinearBvhNode {
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
