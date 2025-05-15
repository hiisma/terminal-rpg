use crate::riengine::{
    event::Event,
    scene::Scene,
    tile::{Tile, TileType},
};

pub fn map01() -> Scene {
    let mut scene = Scene::new();

    scene.id = 1;
    scene.name = String::from("Map 01");
    scene.description = String::from("This is a test map");

    let l: Tile = Tile::new('║', TileType::Wall);
    let b: Tile = Tile::new('█', TileType::Wall);
    let m: Tile = Tile::new('═', TileType::Wall);
    let o: Tile = Tile::new(' ', TileType::Floor);

    let tile_map = vec![
        vec![b, m, m, m, m, m, m, m, m, m, m, m, b],
        vec![l, o, o, o, o, o, o, o, o, o, o, o, l],
        vec![l, o, o, o, o, o, b, b, o, o, o, o, l],
        vec![l, o, o, b, b, o, b, b, o, o, o, o, l],
        vec![l, o, o, b, b, o, b, b, o, o, o, o, l],
        vec![l, o, o, b, b, o, o, o, o, o, o, o, l],
        vec![l, o, o, o, o, o, o, o, o, o, o, o, l],
        vec![b, m, m, m, m, m, m, m, m, m, m, m, b],
    ];

    let e = Event::new();

    let event_map = vec![
        vec![e, e, e, e, e, e, e, e, e, e, e, e, e],
        vec![e, e, e, e, e, e, e, e, e, e, e, e, e],
        vec![e, e, e, e, e, e, e, e, e, e, e, e, e],
        vec![e, e, e, e, e, e, e, e, e, e, e, e, e],
        vec![e, e, e, e, e, e, e, e, e, e, e, e, e],
        vec![e, e, e, e, e, e, e, e, e, e, e, e, e],
        vec![e, e, e, e, e, e, e, e, e, e, e, e, e],
        vec![e, e, e, e, e, e, e, e, e, e, e, e, e],
    ];

    scene.tile_map.set_tile_map(tile_map);
    scene.event_map.set_event_map(event_map);

    scene.player.x = 1;
    scene.player.y = 1;

    scene
}
