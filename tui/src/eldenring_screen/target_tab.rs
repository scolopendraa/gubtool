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
    eldenring::{player, target},
    num_format::{
        Locale::{self},
        ToFormattedString,
    },
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
                TablePane::new_static(&Actions).boxed(),
                TablePane::new_static(&Toggles).boxed(),
            ]),
        }
    }
}

impl Screen for TargetTab {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let [chr_name, hp, poise, paragraph_area, main] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(6),
                Constraint::Fill(1),
            ])
            .areas(rect);

        let lists_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main);

        frame.render_widget(chr_name_paragraph(), chr_name);
        frame.render_widget(hp_line_gauge(), hp);
        frame.render_widget(poise_line_gauge(), poise);
        frame.render_widget(paragraph(), paragraph_area);

        self.pane_manager.draw(frame, &lists_layout);
    }

    fn handle_keys(&mut self, ctx: &mut KeyContext) {
        self.pane_manager.handle_keys(ctx);
    }
}

const ACTIONS: [Command; 7] = [
    Command::Value(ValCmd::I32(&target::Health)),
    Command::Value(ValCmd::F32(&target::HealthPercentage)),
    Command::Unit(&target::Kill),
    Command::Unit(&target::NextPhase),
    Command::Value(ValCmd::U8(&target::RepeatAction)),
    Command::Value(ValCmd::ActArray(&target::ForceActSequence)),
    Command::Unit(&target::ResetPosition),
];

const TOGGLES: [Command; 4] = [
    Command::Toggle(&target::NoDamage),
    Command::Toggle(&target::NoStagger),
    Command::Toggle(&target::DisableAi),
    Command::Toggle(&target::RepeatLastAction),
];

impl_tablecontroller_for_commands!(Actions, ACTIONS);
impl_tablecontroller_for_commands!(Toggles, TOGGLES);

fn hp_line_gauge() -> LineGauge<'static> {
    let current = target::target()
        .chr_ins()
        .and_then(|t| t.get_current_hp())
        .unwrap_or_default();
    let max = target::target()
        .chr_ins()
        .and_then(|t| t.get_max_hp())
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
        .chr_ins()
        .and_then(|t| t.get_current_poise())
        .unwrap_or_default();
    let max = target::target()
        .chr_ins()
        .and_then(|t| t.get_max_poise())
        .unwrap_or_default();

    let label = format!("{:<22}", format!("Poise: {:.1}/{:.1}", current, max));

    line_gauge(label, current as f64, max as f64)
}

fn chr_name_paragraph() -> Paragraph<'static> {
    let name = target::target()
        .chr_ins()
        .map(|t| t.name_from_chr_id())
        .unwrap_or_default();
    Paragraph::new(name)
        .centered()
        .style(Style::from(theme().fg))
        .bold()
}

fn paragraph() -> Paragraph<'static> {
    let poise_timer = target::target()
        .chr_ins()
        .and_then(|t| t.get_poise_timer())
        .unwrap_or_default()
        .abs();
    let last_act = target::target()
        .chr_ins()
        .and_then(|t| t.get_last_act())
        .unwrap_or_default();
    let current_animation = target::target()
        .chr_ins()
        .and_then(|t| t.get_current_animation())
        .unwrap_or_default();
    let distance = target::target()
        .chr_ins()
        .and_then(|target| {
            player::player()
                .chr_ins()
                .and_then(|player| target.get_distance(player))
        })
        .unwrap_or_default();
    Paragraph::new(format!(
        "Reset Timer: {:.2}\n\nLast Act: {last_act}\nCurrent Animation: \
         {current_animation}\nDistance: {:.2}",
        poise_timer, distance
    ))
    .style(Style::from(theme().fg))
}
