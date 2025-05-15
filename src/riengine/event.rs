use super::tile::{Tile, TileType};
#[derive(Clone, Copy)]

pub enum EventType {
    NPC,
    Item,
    Object,
    Trigger,
    None,
}

#[derive(Clone, Copy)]
pub struct EventData {}

#[derive(Clone, Copy)]
pub struct Event {
    pub event_type: EventType,
    pub event_data: EventData,
    pub tile: Tile,
}

impl Event {
    pub fn new() -> Self {
        let event = Event {
            event_type: EventType::None,
            event_data: EventData {},
            tile: Tile {
                tile: ' ',
                tile_type: TileType::Empty,
            },
        };
        event
    }
}

#[derive(Clone)]
pub struct EventMap {
    events: Vec<Vec<Event>>,
}

impl EventMap {
    pub fn new() -> Self {
        let event_map = EventMap {
            events: vec![vec![Event::new()]],
        };
        event_map
    }

    pub fn get_event_map(&self) -> &Vec<Vec<Event>> {
        &self.events
    }

    pub fn set_event_map(&mut self, events: Vec<Vec<Event>>) {
        self.events = events;
    }

    pub fn get_event(&self, x: usize, y: usize) -> Option<&Event> {
        self.events.get(y)?.get(x)
    }
}
