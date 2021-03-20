#[derive(PartialEq, Clone)]
pub enum ActiveBlock {
    Command,
    Block2,
}

pub struct App {
    pub active_block: usize,
    pub all_blocks: Vec<ActiveBlock>,
    pub input: String,
    pub input_index: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            active_block: 0,
            all_blocks: vec![ActiveBlock::Command, ActiveBlock::Block2],
            input: String::new(),
            input_index: 0,
        }
    }

    pub fn next_block(&mut self) {
        self.active_block = (self.active_block + 1) % self.all_blocks.len();
    }

    pub fn active(&self) -> ActiveBlock {
        self.all_blocks[self.active_block].clone()
    }
}
