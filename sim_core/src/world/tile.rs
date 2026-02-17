use crate::world::{fluid::Fluid, material::MaterialType};
use soa_derive::StructOfArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TileStructure {
    #[default]
    Empty, // Air
    Floor(MaterialType), // Walkable ground
    Wall(MaterialType),  // Solid block
    Ramp(MaterialType),  // Slope for moving up Z-levels
}

#[derive(Debug, Clone, Copy, PartialEq, StructOfArray, Default)]
#[soa_derive(Debug, PartialEq)]
pub struct Tile {
    pub structure: TileStructure,
    pub fluid: Option<Fluid>,
    pub heat: f32,
}

impl Tile {
    pub fn is_walkable(&self) -> bool {
        match self.structure {
            TileStructure::Floor(_) => true,
            TileStructure::Ramp(_) => true,
            _ => false,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        matches!(self.structure, TileStructure::Wall(_))
    }
}
