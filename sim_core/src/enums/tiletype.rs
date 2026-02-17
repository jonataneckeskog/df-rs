use crate::enums::materials::MaterialType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Empty,
    Wall(MaterialType),
    Floor(MaterialType),
    Liquid(MaterialType, u8), // Material + depth
}
