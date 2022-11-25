use super::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Map {
    layers: Vec<Layer>,
    #[serde(rename = "tilewidth")]
    tile_width: i32,
    #[serde(rename = "tileheight")]
    tile_height: i32,
}

impl Map {
    pub fn from_tmj(data: &'static str) -> Self {
        serde_json::from_str::<Map>(data).expect("Couldn't deserialize map")
    }

    pub fn index(
        self,
        tileset: &tileset::Tileset,
    ) -> (indexed::Map, LevelLocator) {
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

        let mut map = indexed::Map::new(min_x, min_y, max_x, max_y);
        let mut locator = locator::LevelLocator::default();

        for layer in self.layers {
            for chunk in layer.chunks {
                let mut x = chunk.x;
                let mut y = chunk.y;

                for tile in chunk.data {
                    assert!(x >= chunk.x && x <= chunk.x + chunk.width);
                    assert!(y >= chunk.y && y <= chunk.y + chunk.height);

                    if tile > 0 {
                        map.add(
                            x,
                            y,
                            indexed::Tile::resolve(&layer.name, tileset, tile),
                        );
                    }

                    x += 1;

                    if x >= chunk.x + chunk.width {
                        x = chunk.x;
                        y += 1;
                    }
                }
            }

            for object in layer.objects {
                locator.add(
                    object.name,
                    (object.x / (self.tile_width as f32)).floor() as i32,
                    (object.y / (self.tile_height as f32)).floor() as i32,
                    (object.width as i32) / self.tile_width,
                    (object.height as i32) / self.tile_height,
                );
            }
        }

        (map, locator)
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Layer {
    name: String,
    #[serde(default)]
    chunks: Vec<Chunk>,
    #[serde(default)]
    objects: Vec<Object>,
}

#[derive(Clone, Debug, Deserialize)]
struct Chunk {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    data: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Object {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
}
