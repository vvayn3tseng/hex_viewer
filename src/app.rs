use std::cmp::{max, min};

#[derive(PartialEq, Clone)]
pub enum ActiveBlock {
    Command,
    Viewer,
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
}

pub struct App {
    pub active_block: usize,
    pub all_blocks: Vec<ActiveBlock>,
    pub command_state: CommandState,
}

impl App {
    pub fn new() -> Self {
        App {
            active_block: 0,
            all_blocks: vec![ActiveBlock::Command, ActiveBlock::Viewer],
            command_state: CommandState::new(),
        }
    }

    pub fn next_block(&mut self) {
        self.active_block = (self.active_block + 1) % self.all_blocks.len();
    }

    pub fn active(&self) -> ActiveBlock {
        self.all_blocks[self.active_block].clone()
    }

    pub fn on_char(&mut self, c: char) {
        if self.active() == ActiveBlock::Command {
            self.command_state.on_char(c);
        }
    }

    pub fn on_backspace(&mut self) {
        if self.active() == ActiveBlock::Command {
            self.command_state.on_backspace();
        }
    }

    pub fn on_right(&mut self) {
        if self.active() == ActiveBlock::Command {
            self.command_state.on_right();
        }
    }

    pub fn on_left(&mut self) {
        if self.active() == ActiveBlock::Command {
            self.command_state.on_left();
        }
    }
}
