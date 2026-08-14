use {
    crate::{
        common::helpers::bordered_block,
        event::KeyContext,
        input::input::Input,
        popup::{Popup, PopupState},
        screen::Screen,
        theme::theme,
    },
    crossterm::event::KeyCode,
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        style::Stylize,
        text::Line,
        widgets::Paragraph,
    },
    std::any::TypeId,
};

#[derive(Default)]
pub struct InputPrompt {
    input:       Input,
    prompt:      &'static str,
    prompt_type: Option<TypeId>,
    sender:      Option<tokio::sync::oneshot::Sender<String>>,
    popup_state: PopupState,
}

impl Popup for InputPrompt {
    fn popup_state(&mut self) -> &mut PopupState {
        &mut self.popup_state
    }
    fn screen(&mut self) -> &mut dyn Screen {
        self
    }
    fn popup_rect(&self, frame: &mut Frame) -> Rect {
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Percentage(50),
                Constraint::Fill(1),
            ])
            .split(frame.area());

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(horizontal[1]);

        vertical[1]
    }
    fn close_on_key(&self, _ctx: &mut KeyContext) -> bool {
        false
    }
}

impl Screen for InputPrompt {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let block_theme = if shared::parse_input::can_input_be_parsed_from_type(
            self.prompt_type.unwrap(),
            &self.input.text,
        ) {
            theme().success
        } else {
            theme().error
        };

        let block = bordered_block(None)
            .style(block_theme)
            .title(Line::from(self.prompt).style(block_theme))
            .bg(theme().bg);
        let inner = block.inner(rect);

        self.input.update_width(inner.width);
        let input = Paragraph::new(self.input.to_string()).style(theme().fg);

        self.input.set_cursor(frame, inner);

        frame.render_widget(block, rect);
        frame.render_widget(input, inner);
    }
    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        if ctx.key(KeyCode::Esc) {
            self.prompt_type = None;
            self.sender = None;
            self.close();
        }

        if ctx.key_enter() {
            if let Some(tx) = self.sender.take() {
                let _ = tx.send(self.input.text.to_owned());
            }
            self.prompt_type = None;
            self.close();
        }

        self.input.handle_keys(ctx);
    }
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
        self.popup_state.open();
    }
}
