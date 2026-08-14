use {
    crate::{common::tabs::TabManager, event::KeyContext, panes::PaneManager},
    ratatui::{Frame, layout::Rect},
};

pub trait Screen {
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
    fn handle_keys(&mut self, _ctx: &mut KeyContext) {}
    fn boxed(self) -> Box<Self>
    where Self: Sized {
        Box::new(self)
    }
}

pub trait GameScreen: Screen {
    fn tab_manager(&mut self) -> &mut TabManager;
}

pub trait Tab: Screen {
    fn pane_manager(&mut self) -> &mut PaneManager;
}

impl<T: GameScreen> Screen for T {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let area = self.tab_manager().tab_selector().draw(frame, rect);
        self.tab_manager().current_tab_mut().draw(frame, area);
    }
    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.tab_manager().tab_selector().handle_keys(ctx);
        self.tab_manager().current_tab_mut().handle_keys(ctx);
    }
}

#[macro_export]
macro_rules! impl_game_screen {
    ($($type:ty),*) => {
        $(
            impl crate::screen::GameScreen for $type {
                fn tab_manager(&mut self) -> &mut crate::common::tabs::TabManager {
                    &mut self.tab_manager
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_tab {
    ($($type:ty),*) => {
        $(
            impl crate::screen::Tab for $type {
                fn pane_manager(&mut self) -> &mut crate::panes::PaneManager {
                    &mut self.pane_manager
                }
            }
        )*
    };
}
