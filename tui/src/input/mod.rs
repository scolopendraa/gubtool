pub mod fuzzy_finder;
mod input;
pub mod input_prompt;

use {
    crate::event::{Event, send_event},
    input::Input,
};

pub async fn request_input<T: shared::parse_input::ParseInput + 'static>(
    prompt: Option<&'static str>,
) -> Option<T> {
    let prompt = prompt.unwrap_or("Set new value");
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    send_event(Event::Input((prompt, tx, std::any::TypeId::of::<T>())));
    let reply = rx.await.ok()?;
    T::parse_input(&reply)
}
