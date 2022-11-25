mod grow;

use super::*;

#[derive(Clone, Debug)]
pub struct Map<'a> {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    width: u16,
    tiles: Vec<Vec<Tile<'a>>>,
}

impl<'a> Map<'a> {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        let width = (max_x - min_x).abs();
        let height = (max_y - min_y).abs();

        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            width: width as _,
            tiles: vec![Default::default(); (width * height) as usize],
        }
    }

    pub fn add(
        &mut self,
        x: i32,
        y: i32,
        tiles: impl IntoIterator<Item = Tile<'a>>,
    ) {
        let idx = self
            .xy_to_idx(x, y)
            .unwrap_or_else(|| panic!("Invalid coordinates: {},{}", x, y));

        for tile in tiles {
            self.tiles[idx].push(tile);
        }
    }

    pub fn get(&self, x: i32, y: i32) -> impl Iterator<Item = Tile<'a>> + '_ {
        self.xy_to_idx(x, y)
            .into_iter()
            .flat_map(|idx| self.tiles.get(idx))
            .flatten()
            .copied()
    }

    pub fn geometrize(&self) -> geometrized::Map<'a> {
        let mut map = geometrized::Map::default();

        let mut pending_points: BTreeSet<_> = self
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(idx, tiles)| {
                let (x, y) = self.idx_to_xy(idx);

                tiles.iter().copied().map(move |tile| (x, y, tile))
            })
            .collect();

        while let Some((x, y, tile)) = pending_points.pop_first() {
            match tile {
                Tile::Ceiling(tex) => {
                    let (x1, y1, x2, y2) =
                        grow::rect(&mut pending_points, x, y, tile);

                    map.add(geometrized::Feature::Ceiling {
                        x1,
                        y1,
                        x2,
                        y2,
                        tex,
                    });
                }

                Tile::Floor(tex) => {
                    let (x1, y1, x2, y2) =
                        grow::rect(&mut pending_points, x, y, tile);

                    map.add(geometrized::Feature::Floor {
                        x1,
                        y1,
                        x2,
                        y2,
                        tex,
                    });
                }

                Tile::Wall(tex, rot) => {
                    let (dx, dy) = match rot {
                        0 => (0, -1),
                        1 => (-1, 0),
                        2 => (0, 1),
                        3 => (1, 0),
                        _ => unreachable!(),
                    };

                    if !self.get(x + dx, y + dy).any(|tile| tile.is_floor()) {
                        continue;
                    }

                    let (x1, y1, x2, y2) =
                        grow::line(&mut pending_points, x, y, tile, |x, y| {
                            self.get(x + dx, y + dy).any(|tile| tile.is_floor())
                        });

                    map.add(geometrized::Feature::Wall {
                        x1,
                        y1,
                        x2,
                        y2,
                        rot,
                        tex,
                    });
                }
            }
        }

        log::info!("... found features: {}", map);

        map
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile<'a> {
    Ceiling(&'a str),
    Floor(&'a str),
    Wall(&'a str, u8),
}

impl<'a> Tile<'a> {
    pub fn resolve(
        layer: &str,
        tileset: &'a tileset::Tileset,
        id: u8,
    ) -> Vec<Self> {
        let tile = tileset.tile(id - 1);

        let texture = tile
            .image()
            .strip_prefix("../models/")
            .unwrap()
            .strip_suffix(".png")
            .unwrap();

        match layer {
            "ceilings" => vec![Self::Ceiling(texture)],
            "floors" => vec![Self::Floor(texture)],
            "walls" => (0..4).map(|rot| Self::Wall(texture, rot)).collect(),
            layer => panic!("Unrecognized layer: {}", layer),
        }
    }

    pub fn is_floor(self) -> bool {
        matches!(self, Tile::Floor(_))
    }

    pub fn is_wall(self) -> bool {
        matches!(self, Tile::Wall(_, _))
    }
}
