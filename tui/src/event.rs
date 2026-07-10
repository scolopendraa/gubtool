use crate::app::App;
use config::attach::attach_config_error::AttachConfigError;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use gubtool_core::{
    appdata::{AppDataError, log_error},
    attached::AttachError,
    game_version::Game,
    sys::error::ProcessError,
};
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
    Info((String, InfoType)),
    BlockInputs(bool),
    Input((&'static str, tokio::sync::oneshot::Sender<String>, std::any::TypeId)),
    Search((Vec<Utf32String>, tokio::sync::oneshot::Sender<Option<usize>>)),
    MultiSearch((Vec<Utf32String>, tokio::sync::oneshot::Sender<Vec<usize>>)),
    AppState(Box<dyn FnOnce(&mut App) + Send>),
    Attach,
    ApplyAttach,
    Detach(Game),
}

pub enum InfoType {
    SysError,
    GameError,
    Success,
    Warning,
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

pub fn send_success(text: String) {
    send_event(Event::Info((text, InfoType::Success)));
}

pub trait ResultExt<T> {
    fn send_error(self);
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static
{
    fn send_error(self) {
        let Err(err) = self else { return };
        let (string, info_type) = handle_error(&err);
        send_event(Event::Info((string, info_type)))
    }
}

pub trait AnyhowExt<T> {
    fn send_error(self);
}

impl<T> AnyhowExt<T> for Result<T, anyhow::Error> {
    fn send_error(self) {
        let Err(err) = self else { return };
        let (string, info_type) = handle_error(err.as_ref());
        send_event(Event::Info((string, info_type)))
    }
}

fn handle_error(err: &(dyn std::error::Error + 'static)) -> (String, InfoType) {
    let mut info_type = InfoType::GameError;

    if let Some(proc_error) = err.downcast_ref::<ProcessError>() {
        match proc_error {
            ProcessError::InvalidGame { .. } | ProcessError::InvalidPointer { .. } => (),
            _ => {
                info_type = InfoType::SysError;
                let _ = log_error(&proc_error);
            }
        }
    } else if err.is::<AttachError>() || err.is::<AttachConfigError>() || err.is::<AppDataError>(){
        info_type = InfoType::SysError;
    }

    (err.to_string(), info_type)
}