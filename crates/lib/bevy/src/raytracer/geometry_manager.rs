mod geometry_builder;
mod geometry_updater;

use bevy::prelude::Entity;
use doome_raytracer as rt;

pub use self::geometry_builder::*;
pub use self::geometry_updater::*;

pub struct GeometryManager {
    static_geo: Box<rt::StaticGeometry>,
    static_geo_index: Option<Box<rt::StaticGeometryIndex>>,
    static_geo_owners: Vec<Option<Entity>>,
    dynamic_geo: Box<rt::DynamicGeometry>,
    dynamic_geo_owners: Vec<Entity>,
    uvs: Box<rt::TriangleMappings>,
}

impl GeometryManager {
    pub fn builder(&mut self) -> GeometryBuilder<'_> {
        GeometryBuilder::new(self)
    }

    pub fn updater(&mut self) -> GeometryUpdater<'_> {
        GeometryUpdater::new(self)
    }

    fn alloc_static(
        &mut self,
        entity: Entity,
        triangle: rt::Triangle,
        triangle_uv: rt::TriangleMapping,
    ) {
        log::trace!(
            "Allocating (static): {:?} (triangle={:?}, triangle_uv={:?})",
            entity,
            triangle,
            triangle_uv
        );

        let id = (0..rt::MAX_STATIC_TRIANGLES)
            .map(rt::TriangleId::new_static)
            .find(|id| self.static_geo_owners[id.get()].is_none())
            .expect("Tried to allocate too many static triangles at once");

        self.static_geo.set(id, triangle);
        self.static_geo_index = None;
        self.static_geo_owners[id.get()] = Some(entity);
        self.uvs.set(id.into_any(), triangle_uv);
    }

    fn alloc_dynamic(
        &mut self,
        entity: Entity,
        triangle: rt::Triangle,
        triangle_uv: rt::TriangleMapping,
    ) {
        log::trace!(
            "Allocating (dynamic): {:?} (triangle={:?}, triangle_uv={:?})",
            entity,
            triangle,
            triangle_uv
        );

        let id = self
            .dynamic_geo
            .push(triangle)
            .expect("Tried to allocate too many static triangles at once");

        self.dynamic_geo_owners.push(entity);
        self.uvs.set(id.into_any(), triangle_uv);
    }

    fn update_dynamic(
        &mut self,
        entity: Entity,
        mut for_each: impl FnMut(&mut rt::Triangle, &mut rt::TriangleMapping),
    ) {
        for id in 0..self.dynamic_geo.len() {
            if self.dynamic_geo_owners[id] == entity {
                let id = rt::TriangleId::new_dynamic(id);
                let mut tri = self.dynamic_geo.get(id);
                let mut tri_uv = self.uvs.get(id.into_any());

                for_each(&mut tri, &mut tri_uv);

                self.dynamic_geo.set(id, tri);
                self.uvs.set(id.into_any(), tri_uv);
            }
        }
    }

    pub fn free(&mut self, entity: Entity) {
        log::trace!("Freeing: {:?}", entity);

        self.free_static(entity);
        self.free_dynamic(entity);
    }

    fn free_static(&mut self, entity: Entity) {
        let mut is_dirty = false;

        for id in 0..rt::MAX_STATIC_TRIANGLES {
            if self.static_geo_owners[id] == Some(entity) {
                self.static_geo
                    .set(rt::TriangleId::new_static(id), Default::default());

                self.static_geo_owners[id] = None;

                is_dirty = true;
            }
        }

        if is_dirty {
            self.static_geo_index = None;
        }
    }

    fn free_dynamic(&mut self, entity: Entity) {
        let mut id = 0;

        while id < self.dynamic_geo.len() {
            if self.dynamic_geo_owners[id] == entity {
                let tid = rt::TriangleId::new_dynamic(id);

                self.dynamic_geo.remove(tid);
                self.dynamic_geo_owners.remove(id);
                self.uvs.remove(tid);
            } else {
                id += 1;
            }
        }
    }

    pub fn inner(
        &mut self,
    ) -> Option<(
        &rt::StaticGeometry,
        &rt::StaticGeometryIndex,
        &rt::DynamicGeometry,
        &rt::TriangleMappings,
    )> {
        if self.static_geo_index.is_none() {
            self.static_geo_index =
                rt::GeometryIndexer::index(&self.static_geo);
        }

        Some((
            &self.static_geo,
            self.static_geo_index.as_ref()?,
            &self.dynamic_geo,
            &self.uvs,
        ))
    }
}

impl Default for GeometryManager {
    fn default() -> Self {
        Self {
            static_geo: Default::default(),
            static_geo_index: Default::default(),
            static_geo_owners: vec![None; rt::MAX_STATIC_TRIANGLES],
            dynamic_geo: Default::default(),
            dynamic_geo_owners: Vec::with_capacity(rt::MAX_DYNAMIC_TRIANGLES),
            uvs: Default::default(),
        }
    }
}
