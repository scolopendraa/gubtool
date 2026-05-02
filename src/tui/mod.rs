pub mod app;
mod common;
pub mod ds2;
pub mod er;
mod event;
mod fuzzy_finder;
mod game_screen_selector;
mod help;
mod input;
mod process_selector;
mod theme;

use app::App;

pub fn tui() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let _app_result = App::new().run(terminal);
    ratatui::restore();
    Ok(())
}
