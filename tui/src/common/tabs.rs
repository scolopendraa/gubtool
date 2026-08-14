use {
    crate::{common::helpers::bordered_block, event::KeyContext, screen::Tab, theme::theme},
    crossterm::event::KeyCode,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Offset, Rect},
        symbols,
        widgets::Tabs,
    },
    ratatui_themes::Style,
};

pub struct TabSelector {
    pub current_tab: i64,
    pub title:       Option<&'static str>,
    pub tabs:        &'static [&'static str],
}

impl TabSelector {
    pub fn new(names: &'static [&'static str]) -> Self {
        Self {
            current_tab: 0,
            title:       None,
            tabs:        names,
        }
    }
    pub fn draw(&self, frame: &mut Frame, layout: Rect) -> Rect {
        let [tabs_area, rest] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .areas(layout);

        let tabs = Tabs::new(self.tabs.to_owned())
            .block(bordered_block(self.title))
            .highlight_style(Style::from(theme().accent).bold())
            .select(self.current_tab as usize)
            .divider(symbols::line::VERTICAL);

        frame.render_widget(tabs, tabs_area);
        rest
    }

    pub fn handle_keys(&mut self, ctx: &mut KeyContext) {
        if ctx.key(KeyCode::BackTab) {
            let tabs_len = self.tabs.len() as i64;
            self.current_tab = (self.current_tab + tabs_len - 1) % tabs_len;
        }

        if ctx.key(KeyCode::Tab) {
            let tabs_len = self.tabs.len() as i64;
            self.current_tab = (self.current_tab + tabs_len + 1) % tabs_len;
        }

        if let Some(KeyCode::Char(c)) = ctx.peek_code()
            && let Some(digit) = c.to_digit(10)
            && digit != 0
            && digit <= self.tabs.len() as u32
        {
            ctx.consume();
            self.current_tab = digit as i64 - 1;
        }
    }

    pub fn handle_keys_arrows(&mut self, ctx: &mut KeyContext) {
        if ctx.key_char('h') || ctx.key(KeyCode::Left) {
            let tabs_len = self.tabs.len() as i64;
            self.current_tab = (self.current_tab + tabs_len - 1) % tabs_len;
        }
        if ctx.key_char('l') || ctx.key(KeyCode::Right) {
            let tabs_len = self.tabs.len() as i64;
            self.current_tab = (self.current_tab + tabs_len + 1) % tabs_len;
        }
    }

    pub fn draw_thin(&self, frame: &mut Frame, layout: Rect) {
        let tabs = Tabs::new(self.tabs.to_owned())
            .highlight_style(Style::from(theme().secondary))
            .select(self.current_tab as usize)
            .divider(symbols::DOT);
        frame.render_widget(tabs, layout + Offset::new(1, 0));
    }
}

pub struct TabManager {
    tab_selector: TabSelector,
    tabs:         Vec<Box<dyn Tab>>,
}

impl TabManager {
    pub fn new(
        title: &'static str,
        names: &'static [&'static str],
        tabs: Vec<Box<dyn Tab>>,
    ) -> Self {
        Self {
            tab_selector: TabSelector {
                current_tab: 0,
                title:       Some(title),
                tabs:        names,
            },
            tabs,
        }
    }

    pub fn tab_selector(&mut self) -> &mut TabSelector {
        &mut self.tab_selector
    }

    pub fn current_tab_mut(&mut self) -> &mut Box<dyn Tab> {
        let selected = self.tab_selector.current_tab;
        &mut self.tabs[selected as usize]
    }
}
