use std::collections::HashMap;

use crate::{room::Room, room_location::RoomLocation};

pub struct Dungeon {
    pub map: HashMap<RoomLocation, Room>,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn generate_new_room(&mut self) -> Room {
        Room {}
    }

    pub fn get_room(&mut self, location: RoomLocation) -> &Room {
        let generated_room = self.generate_new_room();
        self.map.entry(location).or_insert(generated_room)
    }
}
