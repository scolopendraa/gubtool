use crate::tui::{common::block, theme::theme};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    symbols,
    widgets::Tabs,
};
use ratatui_themes::Style;

pub struct TabsWidget {
    pub current_tab: i64,
    pub title: &'static str,
    pub tabs: &'static [&'static str],
}

impl TabsWidget {
    pub fn draw(&self, frame: &mut Frame, layout: Rect) -> Rect {
        let [tabs_area, rest] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .areas(layout);

        let tabs = Tabs::new(self.tabs.to_owned())
            .block(block(Some(self.title), None))
            .highlight_style(Style::from(theme().accent).bold())
            .select(self.current_tab as usize)
            .divider(symbols::line::VERTICAL);

        frame.render_widget(tabs, tabs_area);
        rest
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::BackTab, _) => {
                let tabs_len = self.tabs.len() as i64;
                self.current_tab = (self.current_tab + tabs_len - 1) % tabs_len;
            }
            (KeyCode::Tab, _) => {
                let tabs_len = self.tabs.len() as i64;
                self.current_tab = (self.current_tab.clone() + tabs_len + 1) % tabs_len;
            }
            (KeyCode::Char(c), _) if let Some(digit) = c.to_digit(10) => {
                if digit <= self.tabs.len() as u32 {
                    self.current_tab = digit as i64 - 1
                }
            },
            _ => (),
        }
    }
}