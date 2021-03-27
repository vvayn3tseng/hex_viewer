use std::cmp::{max, min};

pub enum CommandResult {
    None,
    Quit,
    Open(String),
}

fn parse_command(input: String) -> CommandResult {
    // let inputs: Vec<&str> = input.split(" ").collect();

    if input == "quit" {
        return CommandResult::Quit;
    }

    CommandResult::None
}

pub struct CommandState {
    pub input: String,
    pub input_index: i32,
}

impl CommandState {
    pub fn new() -> Self {
        CommandState {
            input: String::new(),
            input_index: 0,
        }
    }

    pub fn on_char(&mut self, key: char) {
        self.input.push(key);
        self.input_index += 1;
    }

    pub fn on_backspace(&mut self) {
        let remove_index = max(self.input_index - 1, 0) as usize;

        if self.input.is_empty() || self.input_index == 0 {
            return;
        }

        self.input.remove(remove_index);
        self.on_left();
    }

    pub fn on_left(&mut self) {
        self.input_index = max(self.input_index - 1, 0);
    }

    pub fn on_right(&mut self) {
        self.input_index = min(self.input_index + 1, self.input.len() as i32);
    }

    pub fn on_enter(&mut self) -> CommandResult {
        parse_command(self.input.drain(..).collect())
    }
}
