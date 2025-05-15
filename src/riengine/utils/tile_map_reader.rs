use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::riengine::tile::{Tile, TileType};

pub fn read_tile_map_file(file_path: String) -> Result<Vec<Vec<Tile>>, Box<dyn Error>> {
    let mut tile_map: Vec<Vec<Tile>> = Vec::new();
    let file = File::open(file_path)?;
    let mut buf = BufReader::new(file);
    let readed_line = &mut String::new();
    while buf.read_line(readed_line)? != 0 as usize {
        let iterator = readed_line.trim().chars().into_iter();
        let mut tiles: Vec<Tile> = Vec::new();
        for j in iterator {
            print!("{}", j);
            let readed_tile_type = get_tile_type(j);
            let readed_tile = Tile {
                tile: j,
                tile_type: readed_tile_type,
            };
            tiles.push(readed_tile);
        }
        tile_map.push(tiles);
        readed_line.clear();
    }

    println!("{}, {}", tile_map.len(), tile_map.get(01).unwrap().len());

    Ok(tile_map)
}

fn get_tile_type(t: char) -> TileType {
    if t == ' ' {
        return TileType::Floor;
    }
    TileType::Wall
}
