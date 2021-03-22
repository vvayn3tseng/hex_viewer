mod app;
mod event;
mod ui;

use app::command::CommandResult;
use crossterm::event::KeyCode;
use std::time::Duration;
use std::{io, sync::mpsc};
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
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
                KeyCode::Char(c) => {
                    app.on_char(c);
                }
                KeyCode::Tab => {
                    app.next_block();
                }
                KeyCode::Backspace => {
                    app.on_backspace();
                }
                KeyCode::Left => {
                    app.on_left();
                }
                KeyCode::Right => {
                    app.on_right();
                }
                KeyCode::Enter => match app.on_enter() {
                    CommandResult::Quit => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            },
            event::Event::Tick => {
                // do something
            }
        }
    }

    Ok(())
}
