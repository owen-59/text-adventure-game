use text_input::text;

pub enum InputMessage {
    North,
    South,
    East,
    West,
}

pub struct InputChoice {
    pub name: String,
    pub key: char,
    pub message: InputMessage,
}

pub fn get_input(inputs: &[InputChoice]) -> &InputChoice {
    for input in inputs {
        println!("{}: {}", input.key, input.name);
    }
    let mut u_input;
    loop {
        u_input = text("> ").trim().to_lowercase();
        if let Some(val) = inputs.iter().find(|input| input.key.to_string() == u_input) {
            break val;
        }
        println!("{} is not a valid input.", u_input);
    }
}
