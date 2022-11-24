mod axis;
mod bounding_box;
mod bvh;
mod flat_bvh;
mod serializer;

use std::fmt;
use std::ops::{Index, IndexMut};

use glam::{vec4, Vec3};
use instant::{Duration, Instant};

use self::axis::*;
use self::bounding_box::*;
use self::bvh::*;
use self::flat_bvh::*;
use crate::{StaticGeometry, StaticGeometryIndex, StaticTriangle, Triangle};

type TriangleId = crate::TriangleId<StaticTriangle>;

/// An BVH (SAH-based) geometry indexer.
///
/// Special thanks to:
/// - https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/,
/// - https://github.com/svenstaro/bvh.
#[derive(Default)]
pub struct GeometryIndexer;

impl GeometryIndexer {
    pub fn index(
        geometry: &StaticGeometry,
    ) -> Option<Box<StaticGeometryIndex>> {
        let len = geometry.iter().count();

        log::info!("Indexing geometry; triangles = {}", len);

        if len == 0 {
            return None;
        }

        let (bvh, tt_bvh) = Self::measure(|| Bvh::build(geometry));
        let (fbvh, tt_fbvh) = Self::measure(|| FlatBvh::build(bvh));

        let ((index, index_len), tt_serialize) =
            Self::measure(|| serializer::serialize(fbvh));

        log::info!(
            "Geometry indexed; tt-bvh = {:?}, tt-fbvh = {:?}, tt-serialize = {:?}, index-size = {}",
            tt_bvh,
            tt_fbvh,
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
