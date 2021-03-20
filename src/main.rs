mod app;
mod event;
mod ui;

use crossterm::event::{KeyCode, KeyModifiers};
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

        match rx.recv().unwrap() {
            event::Event::Input(key_event) => match key_event.code {
                KeyCode::Char(c) => {
                    if c == 'q' && key_event.modifiers == KeyModifiers::CONTROL {
                        println!("receive ctrl+q");
                        break;
                    }
                }
                KeyCode::Tab => {
                    app.next_block();
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
