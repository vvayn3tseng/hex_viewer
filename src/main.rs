mod app;
mod event;
mod ui;

use app::command::CommandResult;
use crossterm::{event::KeyCode, terminal::enable_raw_mode};
use std::time::Duration;
use std::{io, sync::mpsc};
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();

    enable_raw_mode().unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(300);

    event::setup_event_loop(tx, tick_rate);

    terminal.clear()?;

    let mut app = app::App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match rx.recv().unwrap_or(event::Event::Tick) {
            event::Event::Input(key_event) => match key_event.code {
                KeyCode::Char(c) => app.on_char(c),
                KeyCode::Tab => app.next_block(),
                KeyCode::Backspace => app.on_backspace(),
                KeyCode::Left => app.on_left(),
                KeyCode::Right => app.on_right(),
                KeyCode::Up => app.on_up(),
                KeyCode::Down => app.on_down(),
                KeyCode::PageUp => app.on_page_up(),
                KeyCode::PageDown => app.on_page_down(),
                KeyCode::Enter => {
                    app.push_erro_msg(String::from(""));
                    match app.on_enter() {
                        CommandResult::Quit => break,
                        CommandResult::Open(path) => app.open_file(path),
                        CommandResult::Jump(offset) => app.on_jump(offset),
                        CommandResult::Error(reason, msg) => {
                            let error_msg = format!("command error {:?}, {}", reason, msg);
                            app.push_erro_msg(error_msg);
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            event::Event::Tick => {
                // do something
            }
        }
    }

    Ok(())
}
