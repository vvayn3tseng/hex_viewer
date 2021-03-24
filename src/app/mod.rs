pub mod command;
pub mod viewer;

use command::{CommandResult, CommandState};
use viewer::ViewerState;

#[derive(PartialEq, Clone)]
pub enum ActiveBlock {
    Command,
    Viewer,
}

pub struct App {
    pub active_block: usize,
    pub all_blocks: Vec<ActiveBlock>,
    pub command_state: CommandState,
    pub viewer_state: ViewerState,
}

impl App {
    pub fn new() -> Self {
        App {
            active_block: 0,
            all_blocks: vec![ActiveBlock::Command, ActiveBlock::Viewer],
            command_state: CommandState::new(),
            viewer_state: ViewerState::new(),
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

    pub fn on_enter(&mut self) -> CommandResult {
        if self.active() == ActiveBlock::Command {
            return self.command_state.on_enter();
        }

        CommandResult::None
    }

    pub fn open_file(&mut self, path: String) {}
}
