use crate::world::{fluid::FluidType, material::MaterialType};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LayerType {
    #[default]
    None, // Air/Empty
    Ground,       // Natural terrain
    Construction, // Built walls/floors
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StructOfArray, Default)]
#[soa_derive(Debug, PartialEq)]
pub struct Tile {
    // --- BOTTOM LAYER (The Earth) ---
    pub ground_elevation: u8, // Height of the dirt
    pub ground_material: MaterialType,

    // Fluid "sits" on the ground layer
    pub fluid_depth: u8, // 0-255. Adds to ground_elevation.
    pub fluid_type: FluidType,

    // --- TOP LAYER ---
    // If bridge_type is None, this layer is empty (Air).
    pub bridge_type: LayerType,
    pub bridge_elevation: u8, // Height of the bridge walking surface.
    pub bridge_material: MaterialType,

    pub heat: u8,
}
