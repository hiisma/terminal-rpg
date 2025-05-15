use crate::riengine::{game::game::Game, tile::TileType};

pub fn clear_screen() {
    print!("\x1b[H\x1b[2J");
}

pub fn draw_screen(game: &Game) {
    let tile_map = game.scene.tile_map.get_tile_map();
    let event_map = game.scene.event_map.get_event_map();
    let player = &game.scene.player;

    for (i, t_row) in tile_map.iter().enumerate() {
        for (j, t_col) in t_row.iter().enumerate() {
            if (player.x == j as i32) && (player.y == i as i32) {
                print!("{}", player.sprite);
            } else if event_map[i][j].tile.tile_type == TileType::Empty {
                print!("{}", t_col.tile);
            } else {
                print!("{}", event_map[i][j].tile.tile);
            }
        }
        println!();
    }
}
