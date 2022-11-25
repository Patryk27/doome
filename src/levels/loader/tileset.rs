use super::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Tileset {
    #[serde(rename = "$value")]
    items: Vec<Item>,
}

impl Tileset {
    pub fn from_tsx(data: &'static str) -> Self {
        quick_xml::de::from_str(data).expect("Couldn't deserialize tileset")
    }

    pub fn tile(&self, id: u8) -> &Tile {
        let id = id.to_string();

        self.items
            .iter()
            .flat_map(|item| item.as_tile())
            .find(|tile| tile.id == id)
            .unwrap_or_else(|| panic!("Unknown tile: {}", id))
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Item {
    Grid,
    Tile(Tile),
}

impl Item {
    pub fn as_tile(&self) -> Option<&Tile> {
        match self {
            Item::Tile(tile) => Some(tile),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tile {
    id: String,
    #[serde(rename = "$value")]
    image: TileImage,
}

impl Tile {
    pub fn image(&self) -> &str {
        &self.image.source
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TileImage {
    source: String,
}
