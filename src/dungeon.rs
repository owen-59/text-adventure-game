use std::collections::HashMap;

use crate::{
    input::{InputChoice, InputMessage, get_input},
    room::Room,
    room_location::RoomLocation,
};

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

    pub fn run_input(&mut self) {
        let inputs = [
            InputChoice {
                name: "go north".to_string(),
                key: 'n',
                message: InputMessage::North,
            },
            InputChoice {
                name: "go east".to_string(),
                key: 'e',
                message: InputMessage::East,
            },
            InputChoice {
                name: "go south".to_string(),
                key: 's',
                message: InputMessage::South,
            },
            InputChoice {
                name: "go west".to_string(),
                key: 'w',
                message: InputMessage::West,
            },
        ];

        let input = get_input(&inputs);

        match input.message {
            InputMessage::North => println!("Heading north."),
            InputMessage::East => println!("Heading east."),
            InputMessage::South => println!("Heading south."),
            InputMessage::West => println!("Heading west."),
        }
    }
}
