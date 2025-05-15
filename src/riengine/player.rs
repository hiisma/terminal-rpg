use super::{math::Vec2D, tile::TileMap};
use std::error::Error;

pub struct Player {
    pub sprite: char,
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new() -> Self {
        let player = Player {
            sprite: '△',
            x: 0,
            y: 0,
        };
        player
    }

    pub fn move_up(&mut self, tile_map: &TileMap) -> Result<(), Box<dyn Error>> {
        self.sprite = '△';
        let pos = Vec2D {
            x: self.x as usize,
            y: (self.y - 1) as usize,
        };
        if tile_map.is_walkable(pos)? {
            self.y -= 1
        }
        Ok(())
    }

    pub fn move_down(&mut self, tile_map: &TileMap) -> Result<(), Box<dyn Error>> {
        self.sprite = '▽';
        let pos = Vec2D {
            x: self.x as usize,
            y: (self.y + 1) as usize,
        };
        if tile_map.is_walkable(pos)? {
            self.y += 1;
        }
        Ok(())
    }

    pub fn move_left(&mut self, tile_map: &TileMap) -> Result<(), Box<dyn Error>> {
        self.sprite = '◁';
        let pos = Vec2D {
            x: (self.x - 1) as usize,
            y: self.y as usize,
        };
        if tile_map.is_walkable(pos)? {
            self.x -= 1;
        }
        Ok(())
    }

    pub fn move_right(&mut self, tile_map: &TileMap) -> Result<(), Box<dyn Error>> {
        self.sprite = '▷';
        let pos = Vec2D {
            x: (self.x + 1) as usize,
            y: self.y as usize,
        };
        if tile_map.is_walkable(pos)? {
            self.x += 1;
        }
        Ok(())
    }
}
