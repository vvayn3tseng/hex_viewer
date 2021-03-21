use crate::app::{ActiveBlock, App};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{
    text::{Span, Spans},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_viewer(f, app, chunks[0]);
    draw_command(f, app, chunks[1]);
}

fn get_border_style(current: ActiveBlock, target: ActiveBlock) -> Style {
    if current == target {
        return Style::default().fg(Color::White);
    }
    Style::default().fg(Color::DarkGray)
}

fn draw_viewer<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let border_style = get_border_style(app.active(), ActiveBlock::Viewer);
    let data: Vec<u8> = vec![
        0x15, 0xfd, 0xa0, 0x00, 0x12, 0x10, 0x80, 0x95, 0xfe, 0x15, 0xfd, 0xa0, 0x00, 0x12, 0x10,
        0x80, 0x95, 0xfe,
    ];
    let mut text = vec![];
    let mut line = vec![];

    let mut count = 0;
    for byte in data {
        let display = format!("{:X}", byte);

        line.push(Span::raw(display));

        count += 1;

        if count == 16 {
            count = 0;
            text.push(Spans::from(line.drain(..).collect::<Vec<Span>>()));
        } else {
            line.push(Span::raw(" "));
        }
    }

    if count != 16 {
        text.push(Spans::from(line.drain(..).collect::<Vec<Span>>()));
    }

    let para = Paragraph::new(text).block(
        Block::default()
            .title("Viewer")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    f.render_widget(para, area);
}

fn draw_command<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let border_style = get_border_style(app.active(), ActiveBlock::Command);

    let command = Paragraph::new(app.command_state.input.as_ref()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Command")
            .border_style(border_style),
    );

    f.set_cursor(
        area.x + app.command_state.input_index as u16 + 1,
        area.y + 1,
    );
    f.render_widget(command, area);
}
