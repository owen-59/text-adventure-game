use crate::{
    door_config::DoorConfig,
    input::{InputChoice, InputMessage, get_input},
    item::Weapon,
};

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn get_direction(door_config: &DoorConfig) -> Direction {
    let inputs = [
        InputChoice {
            name: "go north".to_string(),
            key: 'n',
            message: InputMessage::North,
            show: door_config.north,
        },
        InputChoice {
            name: "go east".to_string(),
            key: 'e',
            message: InputMessage::East,
            show: door_config.east,
        },
        InputChoice {
            name: "go south".to_string(),
            key: 's',
            message: InputMessage::South,
            show: door_config.south,
        },
        InputChoice {
            name: "go west".to_string(),
            key: 'w',
            message: InputMessage::West,
            show: door_config.west,
        },
    ];

    loop {
        let u_input = match get_input(&inputs).message {
            InputMessage::North => Some(Direction::North),
            InputMessage::East => Some(Direction::East),
            InputMessage::South => Some(Direction::South),
            InputMessage::West => Some(Direction::West),
            _ => None,
        };
        if let Some(direction) = u_input {
            break direction;
        }
    }
}

pub fn get_attack_input(weapons: Vec<Weapon>) -> Weapon {
    let inputs: Vec<InputChoice> = weapons
        .iter()
        .map(|weapon| InputChoice {
            name: format!("attack with {} ({} dmg)", weapon.name, weapon.damage),
            message: weapon.message,
            key: weapon.key,
            show: true,
        })
        .collect();

    loop {
        let u_input = match get_input(&inputs).message {
            InputMessage::Punch => Some(Weapon::new_fist()),
            InputMessage::Sword => Some(Weapon::new_sword()),
            _ => None,
        };

        if let Some(attack_input) = u_input {
            break attack_input;
        }
    }
}
