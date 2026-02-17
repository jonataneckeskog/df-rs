// sim_core/src/world/map.rs
use crate::world::tile::Tile;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(w: i32, h: i32, d: i32) -> Self {
        Self {
            width: w,
            height: h,
            depth: d,
            tiles: vec![Tile::default(); (w * h * d) as usize],
        }
    }

    #[inline(always)]
    pub fn get_index(&self, x: i32, y: i32, z: i32) -> Option<usize> {
        if x < 0 || y < 0 || z < 0 || x >= self.width || y >= self.height || z >= self.depth {
            return None;
        }
        Some(((z * self.width * self.height) + (y * self.width) + x) as usize)
    }

    pub fn get_tile(&self, x: i32, y: i32, z: i32) -> Option<&Tile> {
        self.get_index(x, y, z).map(|idx| &self.tiles[idx])
    }
}
