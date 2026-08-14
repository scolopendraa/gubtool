mod app;
mod attach_options;
mod command;
mod common;
mod darksouls2_screen;
mod debug_screen;
mod eldenring_screen;
mod event;
mod game_screen_selector;
mod input;
mod memory_viewer_screen;
mod panes;
mod popup;
mod process_selector;
mod screen;
mod theme;
mod ui_state;

pub fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    crate::app::App::new().run(terminal)?;
    ratatui::restore();
    Ok(())
}
