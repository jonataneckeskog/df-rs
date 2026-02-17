use crate::world::{fluid::Fluid, material::MaterialType};
use soa_derive::StructOfArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TileStructure {
    #[default]
    Empty, // Air
    Floor, // Walkable ground
    Wall,  // Solid block
    Ramp,  // Slope for moving up Z-levels
}

impl TileStructure {
    pub fn is_walkable(&self) -> bool {
        match self {
            TileStructure::Floor => true,
            TileStructure::Ramp => true,
            _ => false,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        matches!(self, TileStructure::Wall)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StructOfArray, Default)]
#[soa_derive(Debug, PartialEq)]
pub struct Tile {
    pub structure: TileStructure,
    pub material: MaterialType,
    pub fluid: Option<Fluid>,
    pub heat: u8,
}
