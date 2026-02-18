use crate::world::{fluid::FluidType, material::MaterialType};
use soa_derive::StructOfArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TileStructure {
    #[default]
    Empty, // Nothing built here; use ground_elevation
    Floor,  // A paved surface at ground_elevation
    Wall,   // A solid block that fills the space above ground_elevation
    Ramp,   // A transition between this XY and a neighbor's elevation
    Stairs, // Vertical movement within this single XY
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
    // --- THE NATURAL FOUNDATION ---
    pub ground_elevation: u8, // The base height of the dirt/rock
    pub ground_material: MaterialType,

    // --- THE "OCCUPANT" (For example a tower construction) ---
    pub structure: TileStructure, // Floor, Wall, Ramp, Stairs, or Empty
    pub structure_material: MaterialType,

    // --- THE FLOODED LAYER ---
    pub fluid_type: FluidType,

    // --- THE OVERLAY (Bridges / Scaffolding) ---
    // In 2.5D, this allows someone to walk *over* a hole or a lower floor.
    pub bridge_type: LayerType,
    pub bridge_elevation: u8,
    pub bridge_material: MaterialType,

    // --- DYNAMIC DATA ---
    pub fluid_depth: u8,
    pub heat: u8,
}
