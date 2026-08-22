use text_input::text;

pub struct InputChoice<'a> {
    pub name: &'a str,
    pub key: char,
    pub callback: Box<dyn Fn()>,
}

pub struct Input<'a> {
    pub inputs: Vec<InputChoice<'a>>,
}

impl<'a> Input<'a> {
    fn _get_input(&self, allow: Vec<String>) -> String {
        let mut input = text("> ");
        while !allow.contains(&input) {
            println!("{} is not recognised.", input);
            input = text("> ");
        }

        input
    }

    pub fn run(&self) {
        for input in &self.inputs {
            println!("{}: {}", input.key, input.name)
        }

        let user_input = self._get_input(
            self.inputs
                .iter()
                .map(|input| input.key.to_string())
                .collect(),
        );

        for input in &self.inputs {
            if input.key.to_string() == user_input {
                (input.callback)();
            }
        }
    }
}
