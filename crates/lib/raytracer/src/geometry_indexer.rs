mod axis;
mod bounding_box;
mod bvh;
mod lbvh;
mod serializer;

use std::fmt;
use std::ops::{Index, IndexMut};

use glam::{vec4, Vec3};
use instant::{Duration, Instant};

use self::axis::*;
use self::bounding_box::*;
use self::bvh::*;
use self::lbvh::*;
use crate::{StaticGeometry, StaticGeometryIndex, StaticTriangle, Triangle};

type TriangleId = crate::TriangleId<StaticTriangle>;

/// An BVH + LBVH geometry indexer.
///
/// Special thanks to:
/// - https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/.
#[derive(Default)]
pub struct GeometryIndexer;

impl GeometryIndexer {
    pub fn index(
        geometry: &StaticGeometry,
    ) -> Option<Box<StaticGeometryIndex>> {
        log::info!("Indexing geometry; triangles = {}", geometry.len());

        if geometry.len() == 0 {
            return None;
        }

        let (bvh, tt_bvh) = Self::measure(|| Bvh::build(geometry));
        let (lbvh, tt_lbvh) = Self::measure(|| LinearBvh::build(bvh));

        let ((index, index_len), tt_serialize) =
            Self::measure(|| serializer::serialize(lbvh));

        log::info!(
            "Geometry indexed; tt-bvh = {:?}, tt-lbvh = {:?}, tt-serialize = {:?}, index-size = {}",
            tt_bvh,
            tt_lbvh,
            tt_serialize,
            index_len,
        );

        Some(Box::new(index))
    }

    fn measure<T>(f: impl FnOnce() -> T) -> (T, Duration) {
        let tt = Instant::now();
        let val = f();
        (val, tt.elapsed())
    }
}
