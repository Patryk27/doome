use std::cmp;
use std::collections::BTreeSet;

use serde::Deserialize;

use super::utils::LevelBuilder;
use crate::prelude::*;

pub struct LevelLoader {
    map: &'static str,
    offset_x: i32,
    offset_y: i32,
}

impl LevelLoader {
    pub fn new(map: &'static str) -> Self {
        Self {
            map,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn offset(mut self, x: i32, y: i32) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    pub fn load(self, lvl: &mut LevelBuilder) {
        log::info!("Loading map");

        serde_json::from_str::<Map>(self.map)
            .unwrap()
            .index()
            .characterize()
            .spawn(lvl, self.offset_x, self.offset_y);

        log::debug!("Map loaded");
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Map {
    layers: Vec<Layer>,
}

impl Map {
    fn index(self) -> IndexedMap {
        log::debug!("Indexing");

        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for layer in &self.layers {
            for chunk in &layer.chunks {
                min_x = cmp::min(min_x, chunk.x);
                min_y = cmp::min(min_y, chunk.y);

                max_x = cmp::max(max_x, chunk.x + chunk.width);
                max_y = cmp::max(max_y, chunk.y + chunk.height);
            }
        }

        // ----

        let mut map = IndexedMap::new(min_x, min_y, max_x, max_y);

        for layer in self.layers {
            for chunk in layer.chunks {
                let mut x = chunk.x;
                let mut y = chunk.y;

                for tile in chunk.data {
                    if tile > 0 {
                        map.add(x, y, Tile::new(tile));
                    }

                    x += 1;

                    if x >= chunk.x + chunk.width {
                        x = chunk.x;
                        y += 1;
                    }

                    if y > chunk.y + chunk.height {
                        unreachable!();
                    }
                }
            }
        }

        map
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Layer {
    chunks: Vec<Chunk>,
}

#[derive(Clone, Debug, Deserialize)]
struct Chunk {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    data: Vec<u8>,
}

// -----

#[derive(Clone, Debug)]
struct IndexedMap {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    width: u16,
    tiles: Vec<Tile>,
}

impl IndexedMap {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        let width = (max_x - min_x).abs();
        let height = (max_y - min_y).abs();

        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            width: width as _,
            tiles: vec![Tile::None; (width * height) as usize],
        }
    }

    fn add(&mut self, x: i32, y: i32, tile: Tile) {
        let idx = self
            .xy_to_idx(x, y)
            .unwrap_or_else(|| panic!("Invalid coordinates: {},{}", x, y));

        self.tiles[idx] = tile;
    }

    fn get(&self, x: i32, y: i32) -> Tile {
        self.xy_to_idx(x, y)
            .map(|idx| self.tiles[idx])
            .unwrap_or(Tile::None)
    }

    fn xy_to_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y
        {
            return None;
        }

        let x = (x - self.min_x) as u16;
        let y = (y - self.min_y) as u16;

        Some((y * self.width + x) as usize)
    }

    fn idx_to_xy(&self, idx: usize) -> (i32, i32) {
        let x = (idx as u16) % self.width;
        let y = (idx as u16) / self.width;

        let x = self.min_x + (x as i32);
        let y = self.min_y + (y as i32);

        (x, y)
    }

    fn characterize(self) -> CharacterizedMap {
        log::debug!("Characterizing");

        let mut map = CharacterizedMap::default();

        let mut pending_points: BTreeSet<_> = self
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(idx, tile)| {
                if *tile == Tile::None {
                    None
                } else {
                    Some(self.idx_to_xy(idx))
                }
            })
            .collect();

        while let Some((x, y)) = pending_points.pop_first() {
            match self.get(x, y) {
                Tile::None => {
                    unreachable!();
                }

                Tile::Floor => {
                    let mut floor = Floor {
                        x1: x,
                        y1: y,
                        x2: x,
                        y2: y,
                    };

                    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                    enum GrowingDir {
                        Up,
                        Right,
                        Down,
                        Left,
                    }

                    let mut dirs = vec![
                        GrowingDir::Up,
                        GrowingDir::Down,
                        GrowingDir::Left,
                        GrowingDir::Right,
                    ];

                    while let Some(dir) = dirs.get(0).cloned() {
                        let points: Vec<_> = match dir {
                            GrowingDir::Up | GrowingDir::Down => (floor.x1
                                ..=floor.x2)
                                .map(|x| {
                                    if dir == GrowingDir::Up {
                                        (x, floor.y1 - 1)
                                    } else {
                                        (x, floor.y2 + 1)
                                    }
                                })
                                .collect(),

                            GrowingDir::Left | GrowingDir::Right => (floor.y1
                                ..=floor.y2)
                                .map(|y| {
                                    if dir == GrowingDir::Left {
                                        (floor.x1 - 1, y)
                                    } else {
                                        (floor.x2 + 1, y)
                                    }
                                })
                                .collect(),
                        };

                        let can_grow = points.iter().all(|&(x, y)| {
                            self.get(x, y) == Tile::Floor
                                && pending_points.contains(&(x, y))
                        });

                        if can_grow {
                            match dir {
                                GrowingDir::Up => {
                                    floor.y1 -= 1;
                                }
                                GrowingDir::Down => {
                                    floor.y2 += 1;
                                }
                                GrowingDir::Left => {
                                    floor.x1 -= 1;
                                }
                                GrowingDir::Right => {
                                    floor.x2 += 1;
                                }
                            }

                            for point in points {
                                pending_points.remove(&point);
                            }
                        } else {
                            dirs.remove(0);
                        }
                    }

                    map.add(CharacterizedMapFeature::Floor(floor));
                }

                Tile::Wall => {
                    for rot in 0..4 {
                        let (dx, dy) = match rot {
                            0 => (0, -1),
                            1 => (-1, 0),
                            2 => (0, 1),
                            3 => (1, 0),
                            _ => unreachable!(),
                        };

                        // TODO growing
                        if self.get(x + dx, y + dy) == Tile::Floor {
                            map.add(CharacterizedMapFeature::Wall(Wall {
                                x1: x + dx,
                                y1: y + dy,
                                x2: x + dx,
                                y2: y + dy,
                                rot,
                            }));
                        }
                    }
                }
            }
        }

        log::info!("... found {} features", map.features.len());

        map
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    None,
    Floor,
    Wall,
}

impl Tile {
    fn new(id: u8) -> Self {
        match id {
            0 => Self::None,
            1 => Self::Floor,
            2 => Self::Wall,
            id => panic!("Unknown tile: {}", id),
        }
    }
}

// -----

#[derive(Default)]
struct CharacterizedMap {
    features: Vec<CharacterizedMapFeature>,
}

impl CharacterizedMap {
    fn add(&mut self, feature: CharacterizedMapFeature) {
        self.features.push(feature);
    }

    fn spawn(self, lvl: &mut LevelBuilder, offset_x: i32, offset_y: i32) {
        log::debug!("Spawning");

        for feature in self.features {
            match feature {
                CharacterizedMapFeature::Floor(floor) => {
                    lvl.floor(
                        floor.x1 + offset_x,
                        floor.y1 + offset_y,
                        floor.x2 + offset_x,
                        floor.y2 + offset_y,
                    )
                    .spawn();

                    lvl.ceiling(
                        floor.x1 + offset_x,
                        floor.y1 + offset_y,
                        floor.x2 + offset_x,
                        floor.y2 + offset_y,
                    )
                    .spawn();
                }

                CharacterizedMapFeature::Wall(wall) => {
                    lvl.wall(
                        wall.x1 + offset_x,
                        wall.y1 + offset_y,
                        wall.x2 + offset_x,
                        wall.y2 + offset_y,
                        wall.rot,
                    )
                    .spawn();
                }
            }
        }
    }
}

enum CharacterizedMapFeature {
    Floor(Floor),
    Wall(Wall),
}

// -----

struct Floor {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

struct Wall {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    rot: u8,
}
