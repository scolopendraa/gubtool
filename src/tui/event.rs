use crate::tui::app::App;
use anyhow::Result;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use nucleo_matcher::Utf32String;
use std::{
    sync::{OnceLock, mpsc},
    thread,
    time::{Duration, Instant},
};

pub enum Event {
    Key(KeyEvent),
    RenderTick,
    BackgroundTick,
    Error(String),
    Search((Vec<Utf32String>, fn(&mut App))),
    Input(fn(String, &mut App)),
}

pub static SENDER: OnceLock<mpsc::Sender<Event>> = OnceLock::new();

pub fn start_event_loop_thread() -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    SENDER.set(tx).unwrap();

    thread::spawn(|| {
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
                && key.kind == event::KeyEventKind::Press {
                send_event(Event::Key(key))
            }

            if last_render_tick.elapsed() >= render_tick_rate {
                send_event(Event::RenderTick);
                last_render_tick = Instant::now();
            }

            if last_background_tick.elapsed() >= background_tick_rate {
                send_event(Event::BackgroundTick);
                last_background_tick = Instant::now();
            }
        }
    });
    rx
}

pub fn send_event(event: Event) {
    SENDER.get().unwrap().clone().send(event).unwrap()
}

#[macro_export]
macro_rules! send_input_event {
    ($text:ident, $app:ident, $body:block) => {
        crate::tui::event::send_event(
            crate::tui::event::Event::Input(
                |$text: String, $app: &mut crate::tui::App| $body
            )
        )
    }
}

pub trait ResultExt<T> {
    fn send_error(self);
}

impl<T> ResultExt<T> for Result<T> {
    fn send_error(self) {
        if let Err(err) = self {
            send_event(Event::Error(err.to_string()))
        }
    }
}