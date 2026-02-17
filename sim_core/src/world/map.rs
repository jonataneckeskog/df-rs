use crate::world::coordinates::Position;
use crate::world::tile::{Tile, TileVec};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: TileVec,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let count = width * height;
        let tiles: TileVec = std::iter::repeat(Tile::default()).take(count).collect();

        Self {
            width,
            height,
            tiles,
        }
    }

    #[inline(always)]
    pub fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }

        let (ux, uy) = (x as usize, y as usize);
        if ux >= self.width || uy >= self.height {
            return None;
        }

        Some(uy * self.width + ux)
    }

    pub fn in_bounds(&self, pos: IVec3) -> bool {
        self.get_index(pos.x, pos.y).is_some()
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        self.get_index(x, y)
            .map(|idx| self.tiles.index(idx).to_owned())
    }

    /// Ergonomic helper for Position component
    pub fn get_at(&self, pos: Position) -> Option<Tile> {
        self.get_tile(pos.0.x, pos.0.y)
    }

    /// This function updates heat without ever loading Structure/Material into CPU cache
    /// using soa_derive.
    pub fn global_heat_decay(&mut self, amount: u8) {
        for heat_val in self.tiles.heat.iter_mut() {
            if *heat_val > 0 {
                *heat_val -= amount;
            }
        }
    }
}
