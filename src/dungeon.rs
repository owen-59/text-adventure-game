use std::collections::HashMap;

use crate::{
    door_config::DoorConfig,
    enemy::{Enemy, EnemyType},
    input_wrappers::{Direction, get_direction},
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

        Room::new(
            DoorConfig {
                north: room_to_north,
                east: room_to_east,
                south: room_to_south,
                west: room_to_west,
            },
            if rand::random_bool(1. / 4.0) {
                Some(Enemy::new(EnemyType::Bat, 128.0))
            } else {
                None
            },
        )
    }

    fn go_direction(&mut self, direction: Direction) {
        self.player.location = self.player.location.copy_with_diff(
            match direction {
                Direction::North => 0,
                Direction::East => 1,
                Direction::South => 0,
                Direction::West => -1,
            },
            match direction {
                Direction::North => 1,
                Direction::East => 0,
                Direction::South => -1,
                Direction::West => 0,
            },
        );

        println!("{}", self.get_room(&self.player.location.clone()).display())
    }

    pub fn get_room(&mut self, location: &RoomLocation) -> &mut Room {
        let generated_room = self.generate_new_room(location);
        self.map.entry(location.clone()).or_insert(generated_room)
    }

    pub fn run_input(&mut self) {
        let location = self.player.location.clone();
        let room = self.get_room(&location);

        let direction = get_direction(&room.door_config);
        match direction {
            Direction::North => {
                if room.door_config.north {
                    self.go_direction(direction);
                } else {
                    println!("Can't go north.");
                }
            }
            Direction::East => {
                if room.door_config.east {
                    self.go_direction(direction);
                } else {
                    println!("Can't go east.");
                }
            }
            Direction::South => {
                if room.door_config.south {
                    self.go_direction(direction);
                } else {
                    println!("Can't go south.");
                }
            }
            Direction::West => {
                if room.door_config.west {
                    self.go_direction(direction);
                } else {
                    println!("Can't go west.");
                }
            }
        }

        println!("{}", self.player.display_items());

        let mut new_player = self.player.clone();

        let location = self.player.location.clone();
        let current_room = self.get_room(&location);

        current_room.cycle(&mut new_player);

        self.player = new_player;
    }

    pub fn display_room(&mut self) {
        let location = self.player.location.clone();
        let room = self.get_room(&location);

        println!("{}", room.display());
    }
}
