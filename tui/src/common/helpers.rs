use {
    crate::theme::{self, theme},
    ratatui::{
        style::{Modifier, Stylize},
        symbols,
        widgets::{Block, LineGauge},
    },
    ratatui_themes::Style,
};

pub fn create_toggle_string(str: &str, state: bool) -> String {
    let toggle = match state {
        true => "[X]",
        false => "[ ]",
    };
    format!("{toggle} {str}")
}

pub fn bordered_block<'a>(title: Option<&'a str>) -> Block<'a> {
    let block = Block::bordered()
        .fg(theme().fg)
        .bg(theme().bg)
        .border_type(theme::BORDER_TYPE);

    match title {
        Some(title) => block.title(title.fg(theme().secondary)),
        None => block,
    }
}

pub fn line_gauge(label: String, current: f64, max: f64) -> LineGauge<'static> {
    let ratio = if max > 0.0 {
        (current / max).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let theme = theme();

    LineGauge::default()
        .label(label)
        .filled_symbol(symbols::block::FULL)
        .filled_style(theme.fg)
        .unfilled_symbol(symbols::block::FULL)
        .unfilled_style(theme.muted)
        .style(theme.fg)
        .ratio(ratio)
}

#[macro_export]
macro_rules! spawn_task {
    ($($body:tt)*) => {
        tokio::spawn(async move {
            $($body)*
        });
    };
}

pub fn item_options_style(show: bool) -> Style {
    if show {
        Style::default()
    } else {
        Style::new().add_modifier(Modifier::CROSSED_OUT)
    }
}
