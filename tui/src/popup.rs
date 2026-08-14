use {
    crate::{event::KeyContext, screen::Screen},
    crossterm::event::{KeyCode, KeyModifiers},
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        widgets::Clear,
    },
};

pub trait Popup {
    fn popup_state(&mut self) -> &mut PopupState;
    fn screen(&mut self) -> &mut dyn Screen;
    fn popup_rect(&self, frame: &mut Frame) -> Rect;

    fn show(&mut self) {
        self.popup_state().open()
    }

    fn close(&mut self) {
        self.popup_state().close()
    }

    fn is_open(&mut self) -> bool {
        self.popup_state().is_open()
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.screen().draw(frame, rect);
    }

    fn draw_if_open(&mut self, frame: &mut Frame) {
        if self.is_open() {
            let rect = self.popup_rect(frame);
            frame.render_widget(Clear, rect);
            self.draw(frame, rect);
        }
    }

    fn close_on_key(&self, ctx: &mut KeyContext) -> bool {
        ctx.key_char('q') | ctx.key_with_modifiers(KeyCode::Esc, KeyModifiers::NONE)
    }

    fn handle_keys_if_open(&mut self, ctx: &mut KeyContext) -> bool {
        if self.is_open() {
            self.screen().handle_keys(ctx);

            if self.close_on_key(ctx) {
                self.close()
            }
            true
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct PopupState {
    is_open: bool,
}

impl PopupState {
    pub fn is_open(&self) -> bool {
        self.is_open
    }
    pub fn open(&mut self) {
        self.is_open = true
    }
    pub fn close(&mut self) {
        self.is_open = false
    }
}

pub fn centered_popup(x: u16, y: u16, layout: Rect) -> Rect {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Percentage(y),
            Constraint::Fill(1),
        ])
        .split(layout);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Percentage(x),
            Constraint::Fill(1),
        ])
        .split(vertical_layout[1])[1]
}
