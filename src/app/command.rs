use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug)]
pub enum CommandErrorReason {
    InvalidArgument,
    UnknownCommand,
}

pub enum CommandResult {
    Error(CommandErrorReason, String),
    None,
    Quit,
    Open(String),
}

struct CommandParser {
    handlers: HashMap<String, fn(Vec<&str>) -> CommandResult>,
}

impl CommandParser {
    pub fn new() -> Self {
        CommandParser {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, command: String, handler: fn(Vec<&str>) -> CommandResult) {
        self.handlers.insert(command, handler);
    }

    pub fn handle(&mut self, input: String) -> CommandResult {
        let mut inputs: Vec<&str> = input.split(" ").collect();

        if inputs.is_empty() {
            return CommandResult::None;
        }

        if !self.handlers.contains_key(&String::from(inputs[0])) {
            return CommandResult::Error(
                CommandErrorReason::UnknownCommand,
                format!("invalid command {}", String::from(inputs[0])),
            );
        }

        self.handlers[&String::from(inputs[0])](inputs.drain(1..).collect())
    }
}

pub struct CommandState {
    pub input: String,
    pub input_index: i32,
    command_parser: CommandParser,
}

fn open_handler(param: Vec<&str>) -> CommandResult {
    if param.len() != 1 {
        return CommandResult::Error(
            CommandErrorReason::InvalidArgument,
            String::from("missing file path"),
        );
    }

    CommandResult::Open(String::from(param[0]))
}

impl CommandState {
    pub fn new() -> Self {
        let mut parser = CommandParser::new();

        parser.add_handler(String::from("quit"), |_| return CommandResult::Quit);
        parser.add_handler(String::from("open"), open_handler);

        CommandState {
            input: String::new(),
            input_index: 0,
            command_parser: parser,
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
        self.input_index = 0;
        self.command_parser.handle(self.input.drain(..).collect())
    }
}
