use {
    crate::input::fuzzy_finder::SearchRequest,
    config::attach::attach_config_error::ApplyAttachError,
    crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    gubtool_core::{
        appdata::{AppDataError, log_error},
        attached::AttachError,
        sys::{ipc::ipc_error::IpcError, sys_error::ProcessError},
    },
    std::{
        sync::{OnceLock, mpsc},
        thread,
        time::Duration,
    },
};

pub enum Event {
    RenderTick,
    Key(KeyContext),
    Info((String, InfoType)),
    Input((&'static str, tokio::sync::oneshot::Sender<String>, std::any::TypeId)),
    SearchRequest(&'static dyn SearchRequest),
    SearchResult(usize),
    GameScreen(gubtool_core::game_version::Game),
    Attach,
}

pub enum InfoType {
    SysError,
    GameError,
    Success,
}

pub struct KeyContext {
    key_event: Option<KeyEvent>,
}

impl KeyContext {
    #[track_caller]
    pub fn key_with_modifiers(&mut self, code: KeyCode, modifiers: KeyModifiers) -> bool {
        let matches = self
            .key_event
            .is_some_and(|key_event| key_event == KeyEvent::new(code, modifiers));

        if matches {
            self.key_event = None;
        }

        matches
    }

    #[track_caller]
    pub fn key(&mut self, code: KeyCode) -> bool {
        let matches = self
            .key_event
            .is_some_and(|key_event| key_event.code == code);

        if matches {
            self.key_event = None;
        }

        matches
    }

    #[track_caller]
    pub fn key_enter(&mut self) -> bool {
        self.key(KeyCode::Enter)
    }

    #[track_caller]
    pub fn key_char(&mut self, c: char) -> bool {
        self.key(KeyCode::Char(c))
    }

    #[track_caller]
    pub fn key_f(&mut self, key: u8) -> bool {
        self.key(KeyCode::F(key))
    }

    pub fn key_any(&mut self) -> bool {
        if self.key_event.is_some() {
            self.key_event = None;
            true
        } else {
            false
        }
    }

    pub fn peek_code(&self) -> Option<KeyCode> {
        self.key_event.map(|k| k.code)
    }

    pub fn consume(&mut self) {
        self.key_event = None
    }

    pub fn consumed(&self) -> bool {
        self.key_event.is_none()
    }
}

static SENDER: OnceLock<mpsc::Sender<Event>> = OnceLock::new();

pub fn start_event_loop_thread() -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    SENDER.set(tx).unwrap();

    thread::spawn(|| {
        let render_tick_rate = Duration::from_millis(50);

        loop {
            if event::poll(render_tick_rate).unwrap()
                && let CEvent::Key(key) = event::read().unwrap()
                && key.kind == event::KeyEventKind::Press
            {
                let ctx = KeyContext {
                    key_event: Some(key),
                };
                send_event(Event::Key(ctx))
            }

            send_event(Event::RenderTick);
        }
    });
    rx
}

#[track_caller]
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
where E: std::error::Error + Send + Sync + 'static
{
    fn send_error(self) {
        let Err(err) = self else {
            return
        };
        let (string, info_type) = handle_error(&err);
        send_event(Event::Info((string, info_type)))
    }
}

pub fn request_search(search_request: &'static dyn SearchRequest) {
    send_event(Event::SearchRequest(search_request));
}

pub trait AnyhowExt<T> {
    fn send_error(self);
}

impl<T> AnyhowExt<T> for Result<T, anyhow::Error> {
    fn send_error(self) {
        let Err(err) = self else {
            return
        };
        let (string, info_type) = handle_error(err.as_ref());
        send_event(Event::Info((string, info_type)))
    }
}

fn handle_error(err: &(dyn std::error::Error + 'static)) -> (String, InfoType) {
    let mut info_type = InfoType::GameError;

    if let Some(proc_error) = err.downcast_ref::<ProcessError>() {
        match proc_error {
            ProcessError::InvalidGame {
                ..
            }
            | ProcessError::NullPointer {
                ..
            } => (),
            _ => {
                info_type = InfoType::SysError;
                let _ = log_error(&proc_error);
            }
        }
    } else if err.is::<AttachError>()
        || err.is::<ApplyAttachError>()
        || err.is::<AppDataError>()
        || err.is::<IpcError>()
    {
        info_type = InfoType::SysError;
    }

    (err.to_string(), info_type)
}
