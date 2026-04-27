use crate::tui::{
    common::{block, centered_rect},
    theme::theme,
};
use ratatui::{
    Frame,
    text::{Line, Span, Text},
    widgets::{Clear, Paragraph, Wrap},
};

const ENTRIES: &[(&str, &str)] = &[
    ("hjkl, ← ↑ ↓ → ", "Navigate list"),
    ("ctrl-hjkl, ← ↑ ↓ → ", "Switch list"),
    ("Enter", "Select"),
    ("s", "Set value"),
    ("f", "Search"),
    ("g", "Jump to first entry"),
    ("G", "Jump to last entry"),
    ("ctrl-u", "Scroll up"),
    ("ctrl-d", "Scroll down"),
    ("1-6", "Switch tab"),
    ("tab", "Select next tab"),
    ("backtab", "Select previous tab"),
    ("f1", "Help"),
    ("f2", "Show valid processes"),
    ("f12", "Change TUI theme"),
];

pub fn draw(frame: &mut Frame) {
    let layout = centered_rect(60, 75, frame.area());
    frame.render_widget(Clear, layout);
    frame.render_widget(paragraph(), layout);
}

fn paragraph() -> Paragraph<'static> {
    let lines: Vec<Line> = ENTRIES
        .iter()
        .map(|f| {
            Line::from(vec![
                Span::raw(format!("{:<22}",f.0)).style(theme().info),
                Span::raw(f.1).style(theme().fg),
            ])
        })
    .collect();
    Paragraph::new(Text::from(lines))
        .wrap(Wrap { trim:true })
        .block(block(Some("Help"), None))
}
