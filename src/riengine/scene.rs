use super::{event::EventMap, player::Player, tile::TileMap};

pub struct Scene {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub tile_map: TileMap,
    pub event_map: EventMap,
    pub player: Player,
}

impl Scene {
    pub fn new() -> Self {
        let scene = Scene {
            id: 0,
            name: String::new(),
            description: String::new(),
            tile_map: TileMap::new(),
            event_map: EventMap::new(),
            player: Player::new(),
        };
        scene
    }
}
