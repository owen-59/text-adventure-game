use crate::{
    door_config::DoorConfig,
    input::{InputChoice, InputMessage, get_input},
};

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub enum AttackInput {
    Punch,
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

pub fn get_attack_input() -> AttackInput {
    let inputs = [InputChoice {
        name: "punch with your fist (5 dmg)".to_string(),
        key: 'p',
        message: InputMessage::Punch,
        show: true,
    }];

    loop {
        let u_input = match get_input(&inputs).message {
            InputMessage::Punch => Some(AttackInput::Punch),
            _ => None,
        };

        if let Some(attack_input) = u_input {
            break attack_input;
        }
    }
}
