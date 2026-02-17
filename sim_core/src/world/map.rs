use crate::world::coordinates::Position;
use crate::world::tile::Tile;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub depth: usize,

    // The size of one Z-level (width * height).
    // Storing this avoids 1 multiplication per lookup.
    xy_area: usize,

    // The flat array of data
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let count = width * height * depth;

        Self {
            width,
            height,
            depth,
            xy_area: width * height,
            tiles: vec![Tile::default(); count],
        }
    }

    #[inline(always)]
    pub fn get_index(&self, x: i32, y: i32, z: i32) -> Option<usize> {
        if x < 0 || y < 0 || z < 0 {
            return None;
        }

        let (ux, uy, uz) = (x as usize, y as usize, z as usize);
        if ux >= self.width || uy >= self.height || uz >= self.depth {
            return None;
        }

        Some((uz * self.xy_area) + (uy * self.width) + ux)
    }

    pub fn in_bounds(&self, pos: IVec3) -> bool {
        self.get_index(pos.x, pos.y, pos.z).is_some()
    }

    pub fn get_tile(&self, x: i32, y: i32, z: i32) -> Option<&Tile> {
        self.get_index(x, y, z).map(|idx| &self.tiles[idx])
    }

    /// Ergonomic helper for Position component
    pub fn get_at(&self, pos: Position) -> Option<&Tile> {
        self.get_tile(pos.0.x, pos.0.y, pos.0.z)
    }

    pub fn get_tile_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut Tile> {
        if let Some(idx) = self.get_index(x, y, z) {
            Some(&mut self.tiles[idx])
        } else {
            None
        }
    }

    /// Ergonomic helper for your Position component (Mutable)
    pub fn get_at_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        self.get_tile_mut(pos.0.x, pos.0.y, pos.0.z)
    }
}
