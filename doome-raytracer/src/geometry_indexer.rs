mod axis;
mod bounding_box;
mod bvh;
mod lbvh;
mod serializer;

use std::fmt;
use std::ops::{Index, IndexMut};

use glam::{vec4, Vec3};

use self::axis::*;
use self::bounding_box::*;
use self::bvh::*;
use self::lbvh::*;
use crate::{Geometry, GeometryIndex, Triangle, TriangleId};

/// An BVH + LBVH geometry indexer.
///
/// Special thanks to:
/// - https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/.
#[derive(Default)]
pub struct GeometryIndexer;

impl GeometryIndexer {
    pub fn index(geometry: &Geometry) -> GeometryIndex {
        log::info!("Indexing geometry; triangles = {}", geometry.len());

        let bvh = Bvh::build(geometry);
        let lbvh = LinearBvh::build(bvh);
        let (index, index_len) = serializer::serialize(lbvh);

        log::info!("Geometry indexed; index size = {}", index_len);

        index
    }
}
