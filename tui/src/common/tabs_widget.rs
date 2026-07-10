use crate::{common::block, theme::theme};
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
    pub title: Option<&'static str>,
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
            .block(block(self.title, None))
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
            (KeyCode::Char(c), _) if matches!(c.to_digit(10), Some(d) if d <= self.tabs.len() as u32 && d != 0) => {
                if let Some(digit) = c.to_digit(10) {
                    self.current_tab = digit as i64 - 1
                }
            }
            _ => (),
        }
    }

    pub fn handle_keys_arrows(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('h') | KeyCode::Left, _) => {
                let tabs_len = self.tabs.len() as i64;
                self.current_tab = (self.current_tab + tabs_len - 1) % tabs_len;
            }
            (KeyCode::Char('l') | KeyCode::Right, _) => {
                let tabs_len = self.tabs.len() as i64;
                self.current_tab = (self.current_tab.clone() + tabs_len + 1) % tabs_len;
            }
            _ => (),
        }
    }

    pub fn current_tab(&self) -> &'static str {
        self.tabs[self.current_tab as usize]
    }

    pub fn draw_thin(&self, frame: &mut Frame, layout: Rect) {
        let tabs = Tabs::new(self.tabs.to_owned())
            .highlight_style(Style::from(theme().secondary))
            .select(self.current_tab as usize)
            .divider(symbols::DOT);
        frame.render_widget(tabs, layout);
    }
}