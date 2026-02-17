use crate::enums::tiletype::TileType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub tile_type: TileType,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Empty,
        }
    }
}
