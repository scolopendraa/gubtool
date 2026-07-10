include!("input.rs");
pub mod fuzzy_finder;
pub mod input_prompt;
pub mod multi_fuzzy_finder;

use crate::event::{Event, send_event};
use nucleo_matcher::Utf32String;

pub async fn request_input<T: shared::parse_input::ParseInput + 'static>(
    prompt: Option<&'static str>,
) -> Option<T> {
    let prompt = prompt.unwrap_or("Set new value");
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    send_event(Event::Input((prompt, tx, std::any::TypeId::of::<T>())));
    let reply = rx.await.ok()?;
    T::parse_input(&reply)
}

pub async fn request_search(entries: Vec<Utf32String>) -> Option<usize> {
    let (tx, rx) = tokio::sync::oneshot::channel::<Option<usize>>();
    send_event(Event::Search((entries, tx)));
    rx.await.ok()?
}

/// Request a free-form string input from the user.
pub async fn request_string(prompt: Option<&'static str>) -> Option<String> {
    let prompt = prompt.unwrap_or("Enter text");
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    send_event(Event::Input((prompt, tx, std::any::TypeId::of::<String>())));
    rx.await.ok()
}

/// Request multi-selection via fuzzy finder. Returns list of selected indices.
pub async fn request_multi_search(entries: Vec<Utf32String>) -> Vec<usize> {
    let (tx, rx) = tokio::sync::oneshot::channel::<Vec<usize>>();
    send_event(Event::MultiSearch((entries, tx)));
    rx.await.ok().unwrap_or_default()
}
