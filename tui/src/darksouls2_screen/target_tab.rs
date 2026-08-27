use {
    crate::{
        common::helpers::line_gauge,
        event::KeyContext,
        impl_tablecontroller_for_commands,
        panes::{PaneManager, TablePane},
        screen::Screen,
        theme::theme,
    },
    crossterm::event::KeyCode,
    darksouls2::{player, target},
    num_format::{Locale, ToFormattedString},
    ratatui::{
        Frame,
        layout::{Constraint, Direction, Layout, Rect},
        style::{Style, Stylize},
        widgets::{LineGauge, Paragraph},
    },
    shared::command::{Command, ValCmd},
};

pub(super) struct TargetTab {
    pub pane_manager: PaneManager,
}

impl TargetTab {
    pub fn new() -> Self {
        TargetTab {
            pane_manager: PaneManager::new(vec![
                TablePane::new_static(&ActionItems).boxed(),
                TablePane::new_static(&ToggleItems).boxed(),
            ]),
        }
    }
}

impl Screen for TargetTab {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let [chr_name, hp, posture, poise, paragraph_area, main] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Fill(1),
            ])
            .areas(rect);

        let lists_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main);

        frame.render_widget(chr_name_paragraph(), chr_name);
        frame.render_widget(hp_line_gauge(), hp);
        frame.render_widget(posture_line_gauge(), posture);
        frame.render_widget(poise_line_gauge(), poise);
        frame.render_widget(paragraph(), paragraph_area);

        self.pane_manager.draw(frame, &lists_layout);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

const ACTION_ITEMS: [Command; 4] = [
    Command::Value(ValCmd::I32(&target::Health)),
    Command::Value(ValCmd::F32(&target::HealthPercentage)),
    Command::Unit(&target::Kill),
    Command::Value(ValCmd::I32(&target::RepeatAction)),
];

const TOGGLE_ITEMS: [Command; 3] = [
    Command::Toggle(&target::RepeatLastAction),
    Command::Toggle(&target::DisableAi),
    Command::Toggle(&target::DisableAiExceptTarget),
];

impl_tablecontroller_for_commands!(ActionItems, ACTION_ITEMS);
impl_tablecontroller_for_commands!(ToggleItems, TOGGLE_ITEMS);

fn hp_line_gauge() -> LineGauge<'static> {
    let current = target::target()
        .chr_ctrl()
        .and_then(|t| t.get_hp())
        .unwrap_or_default();
    let max = target::target()
        .chr_ctrl()
        .and_then(|t| t.max_hp())
        .unwrap_or_default();

    let label = format!(
        "{:<22}",
        format!(
            "Health: {}/{}",
            current.to_formatted_string(&Locale::en),
            max.to_formatted_string(&Locale::en)
        )
    );

    line_gauge(label, current as f64, max as f64)
}

fn poise_line_gauge() -> LineGauge<'static> {
    let current = target::target()
        .chr_ctrl()
        .and_then(|t| t.poise())
        .unwrap_or_default();
    let max = target::target()
        .chr_ctrl()
        .and_then(|t| t.max_poise())
        .unwrap_or_default();
    let vals = if max != 0.0 {
        format!("{:.1}/{:.1}", current, max)
    } else {
        "Immune".to_string()
    };

    let label = format!("{:<22}", format!("Poise: {vals}"));

    line_gauge(label, current as f64, max as f64)
}

fn posture_line_gauge() -> LineGauge<'static> {
    let current = target::target()
        .chr_ctrl()
        .and_then(|t| t.posture())
        .unwrap_or_default();
    let max = target::target()
        .chr_ctrl()
        .and_then(|t| t.max_posture())
        .unwrap_or_default();

    let label = format!(
        "{:<22}",
        format!(
            "Posture: {}/{}",
            (current as i64).to_formatted_string(&Locale::en),
            (max as i64).to_formatted_string(&Locale::en)
        )
    );

    line_gauge(label, current as f64, max as f64)
}

fn chr_name_paragraph() -> Paragraph<'static> {
    let name = target::target()
        .chr_ctrl()
        .map(|t| t.name_from_chr_id())
        .unwrap_or_default();
    Paragraph::new(name)
        .centered()
        .style(Style::from(theme().fg))
        .bold()
}

fn paragraph() -> Paragraph<'static> {
    let last_act = match target::target()
        .chr_ctrl()
        .ok()
        .and_then(|chr| chr.last_act())
    {
        Some(v) => format!("{v}"),
        None => "".to_string(),
    };

    let distance = target::target()
        .chr_ctrl()
        .and_then(|target| {
            player::player()
                .chr_ctrl()
                .and_then(|player| target.get_distance(player))
        })
        .unwrap_or_default();

    Paragraph::new(format!("\nLast Act: {last_act}\nDistance: {:.2}", distance,))
        .style(Style::from(theme().fg))
}
