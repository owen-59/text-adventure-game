use std::collections::HashMap;

use crate::{
    cardinal_direction::CardinalDirection,
    door_config::DoorConfig,
    input::{InputChoice, InputMessage, get_input},
    player::Player,
    room::Room,
    room_location::RoomLocation,
};

enum RoomDoorGenerationStatus {
    NotGenerated,
    HasDoor,
    NoDoor,
}

pub struct Dungeon {
    pub map: HashMap<RoomLocation, Room>,
    pub player: Player,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            player: Player::new(),
        }
    }

    fn generate_new_room(&mut self, location: &RoomLocation) -> Room {
        fn should_create_new_door(status: RoomDoorGenerationStatus) -> bool {
            match status {
                RoomDoorGenerationStatus::NoDoor => false,
                RoomDoorGenerationStatus::HasDoor => true,
                RoomDoorGenerationStatus::NotGenerated => rand::random_ratio(3, 4),
            }
        }

        let room_to_north =
            should_create_new_door(match self.map.get(&location.copy_with_diff(0, 1)) {
                Some(room) => {
                    if room.door_config.south {
                        RoomDoorGenerationStatus::HasDoor
                    } else {
                        RoomDoorGenerationStatus::NoDoor
                    }
                }
                None => RoomDoorGenerationStatus::NotGenerated,
            });

        let room_to_east =
            should_create_new_door(match self.map.get(&location.copy_with_diff(1, 0)) {
                Some(room) => {
                    if room.door_config.west {
                        RoomDoorGenerationStatus::HasDoor
                    } else {
                        RoomDoorGenerationStatus::NoDoor
                    }
                }
                None => RoomDoorGenerationStatus::NotGenerated,
            });
        let room_to_south =
            should_create_new_door(match self.map.get(&location.copy_with_diff(0, -1)) {
                Some(room) => {
                    if room.door_config.north {
                        RoomDoorGenerationStatus::HasDoor
                    } else {
                        RoomDoorGenerationStatus::NoDoor
                    }
                }
                None => RoomDoorGenerationStatus::NotGenerated,
            });
        let room_to_west =
            should_create_new_door(match self.map.get(&location.copy_with_diff(-1, 0)) {
                Some(room) => {
                    if room.door_config.east {
                        RoomDoorGenerationStatus::HasDoor
                    } else {
                        RoomDoorGenerationStatus::NoDoor
                    }
                }
                None => RoomDoorGenerationStatus::NotGenerated,
            });

        Room::new(DoorConfig {
            north: room_to_north,
            east: room_to_east,
            south: room_to_south,
            west: room_to_west,
        })
    }

    fn go_direction(&mut self, direction: CardinalDirection) {
        self.player.location = self.player.location.copy_with_diff(
            match direction {
                CardinalDirection::North => 0,
                CardinalDirection::East => 1,
                CardinalDirection::South => 0,
                CardinalDirection::West => -1,
            },
            match direction {
                CardinalDirection::North => 1,
                CardinalDirection::East => 0,
                CardinalDirection::South => -1,
                CardinalDirection::West => 0,
            },
        );

        println!("{}", self.get_room(&self.player.location.clone()).display())
    }

    pub fn get_room(&mut self, location: &RoomLocation) -> &Room {
        let generated_room = self.generate_new_room(location);
        self.map.entry(location.clone()).or_insert(generated_room)
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

        let location = self.player.location.clone();
        let room = self.get_room(&location);

        match input.message {
            InputMessage::North => {
                if room.door_config.north {
                    self.go_direction(CardinalDirection::North);
                } else {
                    println!("Can't go north.")
                }
            }
            InputMessage::East => {
                if room.door_config.east {
                    self.go_direction(CardinalDirection::East);
                } else {
                    println!("Can't go east.")
                }
            }
            InputMessage::South => {
                if room.door_config.south {
                    self.go_direction(CardinalDirection::South);
                } else {
                    println!("Can't go south.")
                }
            }
            InputMessage::West => {
                if room.door_config.west {
                    self.go_direction(CardinalDirection::West);
                } else {
                    println!("Can't go west.")
                }
            }
        }
    }
}
