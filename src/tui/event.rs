use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

pub enum Event {
    Key(KeyEvent),
    RenderTick,
    BackgroundTick,
    Error(String),
}

pub fn start_event_loop_thread() -> (mpsc::Sender<Event>, mpsc::Receiver<Event>) {
    let (tx, rx) = mpsc::channel();
    let tx_thread = tx.clone();

    thread::spawn(move || {
        let render_tick_rate = Duration::from_millis(30);
        let mut last_render_tick = Instant::now();
        let background_tick_rate = Duration::from_millis(200);
        let mut last_background_tick = Instant::now();

        loop {
            let timeout = render_tick_rate
                .checked_sub(last_render_tick.elapsed())
                .unwrap_or(Duration::ZERO);

            if event::poll(timeout).unwrap()
                && let CEvent::Key(key) = event::read().unwrap()
                && key.kind == event::KeyEventKind::Press
            {
                tx_thread.send(Event::Key(key)).unwrap();
            }

            if last_render_tick.elapsed() >= render_tick_rate {
                tx_thread.send(Event::RenderTick).unwrap();
                last_render_tick = Instant::now();
            }

            if last_background_tick.elapsed() >= background_tick_rate {
                tx_thread.send(Event::BackgroundTick).unwrap();
                last_background_tick = Instant::now();
            }
        }
    });
    (tx, rx)
}
