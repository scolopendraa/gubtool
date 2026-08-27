mod pane_manager;
mod tab;
mod table;

use {
    crate::{event::KeyContext, screen::Screen},
    ratatui::{Frame, layout::Rect},
    shared::command::Command,
};
pub use {pane_manager::*, tab::*, table::*};

pub trait Pane: Screen {
    fn select(&mut self, index: usize);
    fn selected(&self) -> Option<usize>;
    fn current_command(&self) -> Option<Command>;
    fn draw_pane(&mut self, frame: &mut Frame, rect: Rect, active: bool);

    fn draw_active(&mut self, frame: &mut Frame, rect: Rect) {
        self.draw_pane(frame, rect, true);
    }
    fn draw_inactive(&mut self, frame: &mut Frame, rect: Rect) {
        self.draw_pane(frame, rect, false);
    }
}

pub trait TableController {
    fn make_table_view(&self) -> TableView;
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext);
    fn get_command(&self, _selected: usize) -> Option<&shared::command::Command> {
        None
    }
}

pub enum TableControllerSource {
    Static(&'static dyn TableController),
    Owned(Box<dyn TableController>),
}

impl TableController for TableControllerSource {
    fn make_table_view(&self) -> TableView {
        match self {
            Self::Static(p) => p.make_table_view(),
            Self::Owned(p) => p.make_table_view(),
        }
    }
    fn handle_keys_selected(&self, selected: usize, ctx: &mut KeyContext) {
        match self {
            Self::Static(p) => p.handle_keys_selected(selected, ctx),
            Self::Owned(p) => p.handle_keys_selected(selected, ctx),
        }
    }
    fn get_command(&self, selected: usize) -> Option<&shared::command::Command> {
        match self {
            Self::Static(p) => p.get_command(selected),
            Self::Owned(p) => p.get_command(selected),
        }
    }
}

impl From<&'static dyn TableController> for TableControllerSource {
    fn from(value: &'static dyn TableController) -> Self {
        Self::Static(value)
    }
}

impl From<Box<dyn TableController>> for TableControllerSource {
    fn from(value: Box<dyn TableController>) -> Self {
        Self::Owned(value)
    }
}
