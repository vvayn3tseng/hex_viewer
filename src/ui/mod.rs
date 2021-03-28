use crate::app::{ActiveBlock, App};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{
    text::{Span, Spans},
    Frame,
};

const HEADER: &[&str] = &[
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0A", "0B", "0C", "0D", "0E", "0F",
];

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(81),
                Constraint::Percentage(9),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_viewer(f, app, chunks[0]);
    draw_command(f, app, chunks[1]);
    draw_error(f, app, chunks[2]);
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
    let current_offset = app.viewer_state.offset;

    app.viewer_state.height = area.height;

    let mut text = vec![];
    let mut data: &[u8] = &[];

    if app.viewer_state.file_handle.is_some() {
        data = app
            .viewer_state
            .read(current_offset as u64, (area.height * 16) as u64);
    }

    // byte offset header
    let offset_display = format!("{:011X}", current_offset);
    let mut line = vec![];
    line.push(Span::raw(
        (0..offset_display.len() + 2)
            .map(|_| " ")
            .collect::<String>(),
    ));
    for offset in HEADER {
        line.push(Span::raw(offset.clone()));
        line.push(Span::raw(" "));
    }
    text.push(Spans::from(line.drain(..).collect::<Vec<Span>>()));

    // data part
    let mut offset = 0;
    for i in 0..area.height {
        let display = format!("{:011X}  ", current_offset + 16 * i as usize);
        line.push(Span::raw(display));

        if data.len() != 0 {
            let size = if offset + 16 >= data.len() {
                data.len() - offset
            } else {
                16
            };

            for i in 0..size {
                let binary = format!("{:02X}", data[offset + i]);
                line.push(Span::raw(binary));
                line.push(Span::raw(" "));
            }

            offset += size;
        }

        text.push(Spans::from(line.drain(..).collect::<Vec<Span>>()));
    }

    let para = Paragraph::new(text).block(
        Block::default()
            .title("Viewer")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    if app.active() == ActiveBlock::Viewer {
        f.set_cursor(app.viewer_state.cursor.0, app.viewer_state.cursor.1);
    }

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

    if app.active() == ActiveBlock::Command {
        f.set_cursor(
            area.x + app.command_state.input_index as u16 + 1,
            area.y + 1,
        );
    }
    f.render_widget(command, area);
}

fn draw_error<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let error =
        Paragraph::new(app.last_error.as_ref()).block(Block::default().borders(Borders::NONE));

    f.render_widget(error, area);
}
