use {
    crate::{
        common::controls::Control,
        event::{AnyhowExt, KeyContext, ResultExt, send_success},
        panes::{TableController, TablePane, TableView},
        theme::theme,
    },
    ratatui::{
        Frame,
        layout::{Constraint, Offset, Rect},
        style::Stylize,
        text::Span,
        widgets::{Cell, Row},
    },
    shared::event_log::EventLogger,
    std::cell::RefCell,
};

const CONTROLS: [Control; 3] = [
    Control::new("Enter", "Toggle"),
    Control::new("c", "Clear"),
    Control::new("x", "Export"),
];

pub struct EventLogSelectable {
    pub logger: RefCell<Box<dyn EventLogger>>,
}

impl TableController for EventLogSelectable {
    fn make_table_view(&self) -> TableView {
        let _ = self.logger.borrow_mut().poll();

        let rows = self
            .logger
            .borrow()
            .entries()
            .iter()
            .enumerate()
            .rev()
            .map(|(idx, record)| {
                let state = match record.state {
                    true => Span::raw("TRUE").style(theme().success),
                    false => Span::raw("FALSE").style(theme().error),
                };
                Row::new([
                    Cell::from((idx + 1).to_string()),
                    Cell::from(record.event_id.to_string()),
                    Cell::from(state),
                    Cell::from(record.time_stamp.format("%H:%M:%S").to_string()),
                ])
            })
            .collect::<Vec<Row>>();
        let header = Row::new(["Index", "Flag ID", "State", "Time Stamp"]).bold();

        TableView::new(rows).with_header(header).with_widths(&[
            Constraint::Max(7),
            Constraint::Min(12),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
    }
    fn handle_keys_selected(&self, _selected: usize, ctx: &mut KeyContext) {
        if ctx.key_enter() {
            self.logger.borrow().toggle_hook().send_error();
        }

        if ctx.key_char('c') {
            self.logger.borrow_mut().clear().send_error();
        }

        if ctx.key_char('v') {
            self.logger.borrow_mut().toggle_show_duplicates();
        }

        if ctx.key_char('x') {
            self.logger
                .borrow()
                .export()
                .map(|path| send_success(format!("Exported to {}", path)))
                .send_error();
        }
    }
}

pub fn draw_logging_enabled_line(frame: &mut Frame, rect: Rect, enabled: bool) {
    let line = match enabled {
        true => Span::raw("Enabled").style(theme().success),
        false => Span::raw("Disabled").style(theme().error),
    };
    let len = line.width();
    let width = rect.width;
    frame.render_widget(line, rect + Offset::new(width as i32 - len as i32 - 1, 0));
}

impl TablePane {
    pub fn event_logs(logger: impl EventLogger + 'static) -> Self {
        let selectable = EventLogSelectable {
            logger: RefCell::new(Box::new(logger)),
        };

        Self::new_owned(selectable)
            .as_non_selectable()
            .with_title("Event Logs")
            .with_controls(&CONTROLS)
    }
}
