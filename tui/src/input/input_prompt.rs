use crate::input::Input;
use crate::{common::block, theme::theme};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Stylize;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Clear, Paragraph},
};
use std::any::TypeId;

#[derive(Default)]
pub struct InputPrompt {
    input: Input,
    prompt: &'static str,
    prompt_type: Option<TypeId>,
    sender: Option<tokio::sync::oneshot::Sender<String>>,
    pub show: bool,
}

impl InputPrompt {
    pub fn show(
        &mut self,
        prompt: &'static str,
        sender: tokio::sync::oneshot::Sender<String>,
        prompt_type: TypeId,
    ) {
        self.prompt = prompt;
        self.prompt_type = Some(prompt_type);
        self.sender = Some(sender);
        self.input.clear_line();
        self.show = true;
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.input.handle_keys(key);

        match key.code {
            KeyCode::Esc => {
                // Send empty string to the caller to prevent hanging on rx.await
                // when the user cancels the input prompt. The caller's parse_input
                // will reject empty strings for numeric types, returning None.
                if let Some(tx) = self.sender.take() {
                    let _ = tx.send(String::new());
                }
                self.prompt_type = None;
                self.show = false;
            },
            KeyCode::Enter => {
                if let Some(tx) = self.sender.take() {
                    let _ = tx.send(self.input.text.to_owned());
                }
                self.prompt_type = None;
                self.show = false;
            }
            _ => (),
        }
    }

    pub fn draw_popup_checked(&mut self, frame: &mut Frame) {
        if !self.show {
            return;
        }

        let vert = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(frame.area());

        let rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(frame.area().width / 2),
                Constraint::Fill(1),
            ])
            .split(vert[1])[1];

        let block_theme = if shared::parse_input::can_input_be_parsed_from_type(
            self.prompt_type.unwrap(),
            &self.input.text,
        ) {
            theme().success
        } else {
            theme().error
        };

        let block = block(None, None).style(block_theme)
            .title(Line::from(self.prompt).style(block_theme))
            .bg(theme().bg);
        let inner = block.inner(rect);

        self.input.update_width(inner.width);
        let input = Paragraph::new(self.input.to_string())
            .style(theme().fg);

        self.input.set_cursor(frame, inner);

        frame.render_widget(Clear, rect);
        frame.render_widget(block, rect);
        frame.render_widget(input, inner);
    }
}