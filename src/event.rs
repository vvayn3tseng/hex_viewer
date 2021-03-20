use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

pub enum Event<T> {
    Input(T),
    Tick,
}

pub fn setup_event_loop(tx: Sender<Event<KeyEvent>>, tick_rate: Duration) {
    std::thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
}
