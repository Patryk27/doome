use doome_geo::sat::resolve_sat;
use doome_geo::Polygon;
use glam::{vec2, Vec2};
use image::RgbaImage;

pub struct NavData {
    area_start: Vec2,
    area_end: Vec2,

    raster_unit: f32,

    width: isize,
    height: isize,

    // 0 is impassable, 1 is passable
    data: Vec<u8>,
}

type Local = (isize, isize);

impl NavData {
    /// Finds the closest path to point `to`
    ///
    /// returns None if no such path exists:
    ///   - `from` is outside the nav area
    ///   - `from` or `to` is impassable
    ///   - no such path exists due to obstacles
    ///
    /// returns an empty vector if to and from are within the same cell
    pub fn find_path(&self, from: Vec2, to: Vec2) -> Option<Vec<Vec2>> {
        let from = self.global_to_local(from)?;
        let to = self.global_to_local(to)?;

        let (path, _cost) = pathfinding::directed::astar::astar(
            &from,
            |p| self.neighbors(*p).map(|p| (p, 0)),
            |p| Self::distance(*p, to),
            |p| *p == to,
        )?;

        Some(path.into_iter().map(|p| self.local_to_global(p)).collect())
    }

    fn neighbors(&self, p: Local) -> impl Iterator<Item = Local> + '_ {
        const X_OFFSETS: [isize; 3] = [-1, 0, 1];
        const Y_OFFSETS: [isize; 3] = [-1, 0, 1];

        X_OFFSETS
            .iter()
            .flat_map(|x| Y_OFFSETS.iter().map(move |y| (x, y)))
            .filter_map(move |(x, y)| {
                let x = p.0 + x;
                let y = p.1 + y;

                self.is_passable((x, y)).then_some((x, y))
            })
    }

    fn distance(p: Local, to: Local) -> usize {
        p.0.abs_diff(to.0) + p.1.abs_diff(to.1)
    }

    fn is_passable(&self, pos: Local) -> bool {
        let (x, y) = pos;

        if x < 0 || y < 0 {
            return false;
        }

        if x >= self.width || y >= self.height {
            return false;
        }

        let x = x as usize;
        let y = y as usize;

        self.data[y * self.width as usize + x] == 1
    }

    fn global_to_local(&self, global: Vec2) -> Option<Local> {
        if global.x < self.area_start.x || global.y < self.area_start.y {
            return None;
        }

        if global.x >= self.area_end.x || global.y >= self.area_end.y {
            return None;
        }

        let x = (global.x - self.area_start.x) / self.raster_unit;
        let y = (global.y - self.area_start.y) / self.raster_unit;

        let x = x as isize;
        let y = y as isize;

        Some((x, y))
    }

    fn local_to_global(&self, local: Local) -> Vec2 {
        vec2(
            self.area_start.x
                + (local.0 as f32) * self.raster_unit
                + (self.raster_unit / 2.0),
            self.area_start.y
                + (local.1 as f32) * self.raster_unit
                + (self.raster_unit / 2.0),
        )
    }

    pub fn rasterize(&self) -> RgbaImage {
        let mut img = RgbaImage::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x = x as isize;
            let y = y as isize;

            let is_passable = self.is_passable((x, y));

            *pixel = if is_passable {
                image::Rgba([255, 255, 255, 255])
            } else {
                image::Rgba([0, 0, 0, 255])
            };
        }

        img
    }
}

pub struct NavDataBuilder {
    area_start: Option<Vec2>,
    area_end: Option<Vec2>,

    raster_unit: f32,

    polygons: Vec<Polygon>,
}

impl NavDataBuilder {
    pub fn new(raster_unit: f32) -> Self {
        Self {
            area_start: None,
            area_end: None,

            raster_unit,

            polygons: Vec::new(),
        }
    }

    /// Pushes a polygon to the builder
    /// NOTE: the polygon must be transformed into global coordinate space
    pub fn push_polygon(&mut self, polygon: Polygon) {
        for point in polygon.points() {
            if let Some(area_start) = self.area_start {
                self.area_start = Some(vec2(
                    area_start.x.min(point.x),
                    area_start.y.min(point.y),
                ));
            } else {
                self.area_start = Some(*point);
            }

            if let Some(area_end) = self.area_end {
                self.area_end = Some(vec2(
                    area_end.x.max(point.x),
                    area_end.y.max(point.y),
                ));
            } else {
                self.area_end = Some(*point);
            }
        }

        self.polygons.push(polygon);
    }

    pub fn build(self, force_aabb: Option<(Vec2, Vec2)>) -> NavData {
        let area_start =
            self.area_start.or_else(|| force_aabb.map(|f| f.0)).unwrap();

        let area_end =
            self.area_end.or_else(|| force_aabb.map(|f| f.1)).unwrap();

        let width =
            ((area_end.x - area_start.x) / self.raster_unit).ceil() as usize;
        let height =
            ((area_end.y - area_start.y) / self.raster_unit).ceil() as usize;

        let mut data = vec![0; width * height];

        for x in 0..width {
            for y in 0..height {
                let rect_start = vec2(
                    area_start.x + (x as f32 * self.raster_unit),
                    area_start.y + (y as f32 * self.raster_unit),
                );

                let rect_end =
                    rect_start + vec2(self.raster_unit, self.raster_unit);

                let rect = Polygon::rect_start_end(rect_start, rect_end);

                let mut collides_with_any_polygon = false;
                for polygon in self.polygons.iter() {
                    if resolve_sat(polygon, &rect).is_some() {
                        collides_with_any_polygon = true;
                        break;
                    }
                }

                if !collides_with_any_polygon {
                    data[y * width + x] = 1;
                }
            }
        }

        NavData {
            area_start,
            area_end,
            raster_unit: self.raster_unit,
            width: width as isize,
            height: height as isize,
            data,
        }
    }
}
