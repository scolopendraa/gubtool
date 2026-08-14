use {
    crate::{
        common::tabs::TabSelector,
        event::KeyContext,
        panes::{Pane, TablePane},
        screen::Screen,
    },
    ratatui::{Frame, layout::Rect},
};

pub struct TabPane {
    lists:    Vec<TablePane>,
    selector: TabSelector,
}

impl TabPane {
    pub fn new(names: &'static [&'static str], lists: Vec<TablePane>) -> Self {
        Self {
            lists,
            selector: TabSelector::new(names),
        }
    }

    fn current_list(&self) -> &TablePane {
        &self.lists[self.selector.current_tab as usize]
    }

    fn current_list_mut(&mut self) -> &mut TablePane {
        &mut self.lists[self.selector.current_tab as usize]
    }
}

impl Pane for TabPane {
    fn select(&mut self, index: usize) {
        self.current_list_mut().select(index);
    }
    fn selected(&self) -> Option<usize> {
        self.current_list().selected()
    }
    fn current_command(&self) -> Option<&shared::command::Command> {
        self.current_list().current_command()
    }
    fn draw_pane(&mut self, frame: &mut Frame, rect: Rect, active: bool) {
        self.lists[self.selector.current_tab as usize].draw_pane(frame, rect, active);
        self.selector.draw_thin(frame, rect);
    }
}

impl Screen for TabPane {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.draw_pane(frame, rect, true);
    }
    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.selector.handle_keys_arrows(ctx);
        self.current_list_mut().handle_keys(ctx);
    }
}
