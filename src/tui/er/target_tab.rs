use crate::{
    config::ui_state::UiState,
    er::{
        chr_ins::{ChrIns, ChrInsExt},
        player, target,
    },
    send_input_event,
    tui::{
        common::{StrExt, parse_act_sequence, stateful_list::StatefulList, tab_state::TabState, tabs_list},
        er::ErInfo,
        event::ResultExt,
        theme::theme,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use num_format::{Locale, ToFormattedString};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{LineGauge, List, ListItem, Paragraph},
};

enum ActionsItems {
    Kill,
    SetHealth,
    SetHealthPercentage,
    ForceAct,
    ForceActSequence,
    ResetPosition,
}

enum TogglesItems {
    NoDamage,
    NoStagger,
    DisableAi,
    RepeatAction,
}

const ACTIONS_IDX: usize = 0;
const TOGGLES_IDX: usize = 1;

pub struct TargetTab {
    tab: TabState,
    pub hp_val: i32,
    pub hp_percentage: i32,
    pub act: i32,
    pub act_array: Vec<i32>,
}

impl TargetTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 2];
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        TargetTab {
            tab: TabState::new(list_states),
            hp_val: 1,
            hp_percentage: 50,
            act: 1,
            act_array: vec![],
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect, er: &ErInfo) {
        let [chr_name, hp, poise, paragraph_area, main] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(6),
                Constraint::Fill(1),
            ])
            .areas(layout);

        let lists_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ])
            .split(main);

        frame.render_widget(Self::chr_name_paragraph(&er.target_ins), chr_name);
        frame.render_widget(Self::hp_line_gauge(&er.target_ins), hp);
        frame.render_widget(Self::poise_line_gauge(&er.target_ins), poise);
        frame.render_widget(Self::paragraph(&er.target_ins), paragraph_area);

        frame.render_stateful_widget(
            ActionsItems::list(&self),
            lists_layout[ACTIONS_IDX],
            &mut self.tab.get_list_state(ACTIONS_IDX),
        );
        frame.render_stateful_widget(
            TogglesItems::list(self, &er.target_ins),
            lists_layout[TOGGLES_IDX],
            &mut self.tab.get_list_state(TOGGLES_IDX),
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent, er: &ErInfo) {
        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_select(&er.target_ins),
            _ => (),
        }
    }

    fn handle_input(&self) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].set_input(),
                _ => (),
            }
        }
    }

    fn handle_select(&self, target_ins: &ChrIns) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].execute(self, target_ins),
                TOGGLES_IDX => TogglesItems::ARRAY[selected_index].execute(target_ins),
                _ => (),
            }
        }
    }

    fn hp_line_gauge(target_ins: &ChrIns) -> LineGauge<'static> {
        let current = target_ins.get_current_hp().unwrap_or_default();
        let max = target_ins.get_max_hp().unwrap_or_default();
        LineGauge::default()
            .label(format!(
                "{:<22}", format!("Health: {}/{}",
                    current.to_formatted_string(&Locale::en),
                    max.to_formatted_string(&Locale::en)
                )
            ))
            .filled_style(Style::from(theme().fg).bg(theme().fg).bold())
            .ratio(if max > 0 { (current as f64 / max as f64).clamp(0.0, 1.0) } else { 0.0 })
        .style(Style::from(theme().fg))
    }

    fn poise_line_gauge(target_ins: &ChrIns) -> LineGauge<'static> {
        let current = target_ins.get_current_poise().unwrap_or_default();
        let max = target_ins.get_max_poise().unwrap_or_default();
        LineGauge::default()
            .label(format!(
                "{:<22}", format!("Poise: {:.1}/{:.1}", current, max)))
            .filled_style(Style::from(theme().fg).bg(theme().fg).bold())
            .ratio(if max > 0.0 { (current as f64 / max as f64).clamp(0.0, 1.0) } else { 0.0 })
        .style(Style::from(theme().fg))
    }

    fn chr_name_paragraph(target_ins: &ChrIns) -> Paragraph<'static> {
        Paragraph::new(target_ins.name_from_chr_id())
        .centered()
        .style(Style::from(theme().fg))
        .bold()
    }

    fn paragraph(target_ins: &ChrIns) -> Paragraph<'static> {
        let poise_timer = target_ins.get_poise_timer().unwrap_or_default().abs();
        let last_act = target_ins.get_last_act().unwrap_or_default();
        let current_animation = target_ins.get_current_animation().unwrap_or_default();
        let distance = target_ins
            .get_distance(&player::player_ins())
            .unwrap_or_default();
        Paragraph::new(format!(
            "Reset Timer: {:.1}\n\nLast Act: {last_act}\nCurrent Animation: {current_animation}\nDistance: {:.1}",
            poise_timer, distance
        ))
        .style(Style::from(theme().fg))
    }
}

impl ActionsItems {
    fn execute(&self, target_tab: &TargetTab, target_ins: &ChrIns) {
        match self {
            Self::Kill => target_ins.set_hp(0).send_error(),
            Self::SetHealth => target_ins.set_hp(target_tab.hp_val).send_error(),
            Self::SetHealthPercentage => target_ins.set_hp_pct(target_tab.hp_percentage).send_error(),
            Self::ForceAct => target_ins.force_act(target_tab.act).send_error(),
            Self::ResetPosition => target_ins.reset_position().send_error(),
            Self::ForceActSequence => {
                target::force_act_sequence(
                    target_tab.act_array.to_owned(),
                    target_ins.npc_think_param_id().unwrap_or_default(),
                ).send_error()
            }
        }
    }
    fn set_input(&self) {
        match self {
            Self::SetHealth => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.elden_ring.target.hp_val = v;
                        UiState::update_er(|c| { c.target_set_health = v; }).ok();
                    }
                })
            },
            Self::SetHealthPercentage => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.elden_ring.target.hp_percentage = v;
                        UiState::update_er(|c| { c.target_set_health_pct = v; }).ok();
                    }
                })
            },
            Self::ForceAct => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.elden_ring.target.act = v;
                        UiState::update_er(|c| { c.target_act = v; }).ok();
                    }
                })
            },
            Self::ForceActSequence => {
                send_input_event!(text, app, {
                    let result = parse_act_sequence(text);
                    if let Ok(v) = result {
                        app.elden_ring.target.act_array = v.clone();
                        UiState::update_er(|c| { c.target_act_array = v; }).ok();
                    } else {
                        result.send_error()
                    }
                })
            },
            _ => (),
        }
    }
    fn to_list_item(&self, target: &TargetTab) -> ListItem<'_> {
        let text = match self {
            Self::Kill => "Kill".to_string(),
            Self::SetHealth => format!("Set Health ({})", target.hp_val),
            Self::SetHealthPercentage => format!("Set % Health ({}%)", target.hp_percentage),
            Self::ForceAct => format!("Force Act ({})", target.act),
            Self::ForceActSequence => format!("Force Act Sequence {:?}", target.act_array),
            Self::ResetPosition => "Reset Position".to_string(),
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
        Self::Kill,
        Self::SetHealth,
        Self::SetHealthPercentage,
        Self::ForceAct,
        Self::ForceActSequence,
        Self::ResetPosition,
    ];
    fn list(target_tab: &TargetTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(target_tab)).collect();
        tabs_list(items, None, &target_tab.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self, target_ins: &ChrIns) {
        match self {
            Self::NoDamage => {
                let new_state = !target_ins.is_no_damage().unwrap_or_default();
                target_ins.set_no_damage(new_state).send_error()
            }
            Self::RepeatAction => {
                let new_state = !target_ins.is_repeat_act().unwrap_or_default();
                target_ins.set_repeat_act(new_state).send_error()
            }
            Self::DisableAi => {
                let new_state = !target_ins.is_disable_ai().unwrap_or_default();
                target_ins.set_disable_ai(new_state).send_error()
            }
            Self::NoStagger => {
                target::toggle_stagger_hook().send_error()
            }
        }
    }
    fn to_list_item(&self, target_ins: &ChrIns) -> ListItem<'_> {
        let text = match self {
            Self::NoDamage => {
                let state = target_ins.is_no_damage().unwrap_or_default();
                "No Damage".create_toggle_str(state)
            }
            Self::NoStagger => {
                let state = target::is_stagger_hook_active().unwrap_or_default();
                "No Stagger".create_toggle_str(state)
            }
            Self::DisableAi => {
                let state = target_ins.is_disable_ai().unwrap_or_default();
                "Disable AI".create_toggle_str(state)
            }
            Self::RepeatAction => {
                let state = target_ins.is_repeat_act().unwrap_or_default();
                "Repeat Action".create_toggle_str(state)
            }
        };
        ListItem::from(text)
    }
    const ARRAY: &[TogglesItems] = &[
        Self::NoDamage,
        Self::NoStagger,
        Self::DisableAi,
        Self::RepeatAction,
    ];

    fn list(target_tab: &TargetTab, target_ins: &ChrIns) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(target_ins)).collect();
        tabs_list(items, None, &target_tab.tab, TOGGLES_IDX)
    }
}