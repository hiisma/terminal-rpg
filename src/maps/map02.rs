use std::str::FromStr;

use crate::riengine::{event::Event, scene::Scene, utils::tile_map_reader::read_tile_map_file};

pub fn map02() -> Scene {
    let mut scene = Scene::new();

    scene.id = 2;
    scene.name = String::from("Map 02");
    scene.description = String::from("This is the second map test");

    let tile_map =
        read_tile_map_file(String::from_str("./resources/map02.riemap").unwrap()).unwrap();

    println!("{}, {}", tile_map.len(), tile_map.get(01).unwrap().len());

    let e = Event::new();
    let mut event_map: Vec<Vec<Event>> = Vec::new();

    for _ in 0..tile_map.len() {
        let mut events: Vec<Event> = Vec::new();
        for _ in 0..tile_map[0].len() {
            events.push(e);
        }
        event_map.push(events);
    }

    scene.tile_map.set_tile_map(tile_map);
    scene.event_map.set_event_map(event_map);

    scene.player.x = 1;
    scene.player.y = 1;
    scene
}
