use std::error::Error;

use super::math::Vec2D;

// TileType enum represents the type of tile
#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
}

// Tile struct represents a single tile in the tilemap

#[derive(Clone, Copy)]
pub struct Tile {
    pub tile: char,
    pub tile_type: TileType,
}

// TileMap struct represents the entire tilemap
#[derive(Clone)]
pub struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

// Tile implementation
impl Tile {
    pub fn new(tile: char, tile_type: TileType) -> Self {
        Tile { tile, tile_type }
    }

    pub fn get_tile_type(&self) -> TileType {
        self.tile_type.clone()
    }

    pub fn get_tile(&self) -> char {
        self.tile
    }
}

impl TileMap {
    pub fn new() -> Self {
        let tile_map = TileMap {
            tiles: vec![vec![Tile::new(' ', TileType::Empty)]],
        };
        tile_map
    }

    // Get the tilemap
    pub fn get_tile_map(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    // Get a tile from the tilemap
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y)?.get(x)
    }

    pub fn set_tile_map(&mut self, tile_map: Vec<Vec<Tile>>) {
        self.tiles = tile_map;
    }

    pub fn get_width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.tiles.len()
    }

    pub fn is_walkable(&self, pos: Vec2D<usize>) -> Result<bool, Box<dyn Error>> {
        // Check boundaries
        if let Some(row) = self.tiles.get(pos.y) {
            if let Some(tile) = row.get(pos.x) {
                return Ok(tile.tile_type == TileType::Floor);
            }
        }
        Err("Position out of bounds".into())
    }
}
