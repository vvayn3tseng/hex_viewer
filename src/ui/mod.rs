use crate::app::{ActiveBlock, App};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui::Frame;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let mut block = Block::default()
        .title("Command")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    if app.active() == ActiveBlock::Command {
        block = block.border_style(Style::default().fg(Color::White));
    }

    f.render_widget(block, chunks[0]);

    let mut block = Block::default()
        .title("Block 2")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    if app.active() == ActiveBlock::Block2 {
        block = block.border_style(Style::default().fg(Color::White));
    }

    f.render_widget(block, chunks[1]);
}
