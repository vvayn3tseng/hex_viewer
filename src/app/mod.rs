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
    pub last_error: String,
}

impl App {
    pub fn new() -> Self {
        App {
            active_block: 0,
            all_blocks: vec![ActiveBlock::Viewer, ActiveBlock::Command],
            command_state: CommandState::new(),
            viewer_state: ViewerState::new(),
            last_error: String::from(""),
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
        match self.active() {
            ActiveBlock::Command => self.command_state.on_right(),
            ActiveBlock::Viewer => self.viewer_state.on_right(),
        }
    }

    pub fn on_left(&mut self) {
        match self.active() {
            ActiveBlock::Command => self.command_state.on_left(),
            ActiveBlock::Viewer => self.viewer_state.on_left(),
        }
    }

    pub fn on_up(&mut self) {
        match self.active() {
            ActiveBlock::Viewer => self.viewer_state.on_up(),
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        match self.active() {
            ActiveBlock::Viewer => self.viewer_state.on_down(),
            _ => {}
        }
    }

    pub fn on_enter(&mut self) -> CommandResult {
        if self.active() == ActiveBlock::Command {
            return self.command_state.on_enter();
        }

        CommandResult::None
    }

    pub fn on_page_down(&mut self) {
        if self.active() == ActiveBlock::Viewer {
            self.viewer_state.on_page_down();
        }
    }

    pub fn on_page_up(&mut self) {
        if self.active() == ActiveBlock::Viewer {
            self.viewer_state.on_page_up();
        }
    }

    pub fn open_file(&mut self, path: String) {
        self.viewer_state.open(path);
    }

    pub fn push_erro_msg(&mut self, msg: String) {
        self.last_error = msg;
    }
}
