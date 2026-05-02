use crate::{
    config::ui_state::UiState,
    er::{
        event::{self, get_dlc_clear, mass_revive},
        resources::bosses::bosses_array,
    },
    send_input_event,
    tui::{
        app::App,
        common::{
            StrExt, block, blockless_list, label_list, stateful_list::StatefulList,
            tab_state::TabState, tabs_list,
        },
        er::ErInfo,
        event::{Event, ResultExt, send_event},
        theme::{self, theme},
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};
use std::thread;

enum CommandsItems {
    Event,
    FightFortissax,
    FightEldenBeast,
    UnlockMetyr,
    DlcClear,
}

enum ReviveOptions {
    FirstEncounter,
    WarpOnRevive,
    MassRevive,
}

const COMMANDS_IDX: usize = 0;
const BOSSES_IDX: usize = 1;
pub const REVIVE_OPTIONS_IDX: usize = 2;

pub struct EventTab {
    pub tab: TabState,
    pub event: Option<u32>,
    pub first_encounter: bool,
    pub warp: bool,
}

impl EventTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[COMMANDS_IDX] = StatefulList::new(CommandsItems::ARRAY.len());
        list_states[BOSSES_IDX] = StatefulList::new(0);
        list_states[REVIVE_OPTIONS_IDX] = StatefulList::new(ReviveOptions::ARRAY.len());
        EventTab {
            tab: TabState::new(list_states),
            event: None,
            first_encounter: false,
            warp: true,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect, er: &ErInfo) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(layout);

        let block = self.revive_block(er);
        frame.render_widget(&block, layout[BOSSES_IDX]);
        let inner = block.inner(layout[BOSSES_IDX]);

        let [revive_list, revive_options] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(2),
                Constraint::Length(4),
            ])
            .areas(inner);

        frame.render_stateful_widget(
            CommandsItems::list(self, er.dlc),
            layout[COMMANDS_IDX],
            &mut self.tab.get_list_state(COMMANDS_IDX),
        );

        let [boss_name, boss_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(44),
                Constraint::Max(33),
            ])
            .areas(revive_list);

        let (boss_names, boss_areas) = self.bosses_list(er.dlc);
        frame.render_stateful_widget(
            boss_names,
            boss_name,
            &mut self.tab.get_list_state(BOSSES_IDX),
        );
        frame.render_stateful_widget(
            boss_areas,
            boss_area,
            &mut self.tab.get_list_state(BOSSES_IDX),
        );
        frame.render_stateful_widget(
            ReviveOptions::list(self),
            revive_options,
            &mut self.tab.get_list_state(REVIVE_OPTIONS_IDX),
        );
    }

    fn revive_block(&self, er: &ErInfo) -> Block<'static> {
        let style = if self.tab.current_list == BOSSES_IDX {
            self.tab.block_style(BOSSES_IDX)
        } else {
            self.tab.block_style(REVIVE_OPTIONS_IDX)
        };
        block(Some("Boss Revives"), Some(style))
            .title(self.revive_status_line(er.loaded, er.dlc).right_aligned())
    }

    pub fn handle_keys(&mut self, key: KeyEvent, er: &ErInfo) {
        if self.tab.current_list == BOSSES_IDX {
            self.tab.set_length(BOSSES_IDX, bosses_array(er.dlc).len())
        }
        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_select(er.dlc),
            KeyCode::Char('f') if matches!(self.tab.current_list, BOSSES_IDX | REVIVE_OPTIONS_IDX) => {
                let list = bosses_array(er.dlc).iter()
                    .map(|boss| Utf32String::from(format!("{}|{}", boss.name, boss.main_area)))
                    .collect();
                let function = |app: &mut App| {
                    app.elden_ring.event.tab.set_list_selected(
                        BOSSES_IDX,
                        app.fuzzy_finder.selected_idx().unwrap(),
                    )
                };
                send_event(Event::Search((list, function)))
            }
            _ => (),
        }
    }

    fn handle_input(&self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                COMMANDS_IDX => CommandsItems::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }

    fn handle_select(&mut self, dlc: bool) {
        let Some(selected) = self.tab.get_list_selected(self.tab.current_list) else { return };

        if self.tab.current_list == COMMANDS_IDX {
            CommandsItems::array(dlc)[selected].execute(self)
        }
        if self.tab.current_list == REVIVE_OPTIONS_IDX {
            ReviveOptions::ARRAY[selected].execute(self, dlc)
        }
        if self.tab.current_list == BOSSES_IDX {
            let first_encounter = self.first_encounter;
            let warp = self.warp;
            thread::spawn(move || {
                bosses_array(dlc)[selected].revive(first_encounter, warp).send_error()
            });
        }
    }

    fn bosses_list(&self, dlc: bool) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = bosses_array(dlc).iter()
            .map(|boss| (
                    ListItem::from(boss.name),
                    ListItem::from(Line::raw(boss.main_area)).fg(theme().muted)
            ))
            .collect();
        (
            blockless_list(items.0, &self.tab, BOSSES_IDX),
            label_list(items.1, &self.tab, BOSSES_IDX)
        )
    }

    fn revive_status_line(&self, loaded: bool, dlc: bool) -> Line<'static> {
        let selected_idx = self.tab.lists_states[BOSSES_IDX].selected().unwrap_or_default();
        let boss = bosses_array(dlc)[selected_idx];
        let mut style = Style::from(theme().success);
        let text = if !loaded {
            "".to_string()
        } else {
            boss.revive_status().to_string()
        };
        if self.tab.current_list != BOSSES_IDX {
            style = Style::from(theme().fg)
        } else if text == event::DEAD {
            style = Style::from(theme().error)
        }
        Line::from(text)
            .style(style)
    }
}

impl CommandsItems {
    fn execute(&self, event_tab: &EventTab) {
        match self {
            Self::Event => {
                if let Some(event) = event_tab.event {
                    let new_state = !event::get_event(event).unwrap_or_default();
                    event::set_event(event, new_state).send_error()
                }
            }
            Self::FightFortissax => {
                event::fight_fortissax().send_error()
            }
            Self::FightEldenBeast => {
                event::fight_elden_beast().send_error()
            }
            Self::UnlockMetyr => {
                event::unlock_metyr().send_error()
            }
            Self::DlcClear => {
                let new_state = !get_dlc_clear().unwrap_or_default();
                event::set_dlc_clear(new_state).send_error()
            }
        }
    }
    fn set_input(&self) {
        match self {
            Self::Event => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.elden_ring.event.event = Some(v);
                        UiState::update_er(|c| { c.event = Some(v); }).ok();
                    } else if text.is_empty() {
                        app.elden_ring.event.event = None;
                        UiState::update_er(|c| { c.event = None; }).ok();
                    }
                })
            },
            _ => (),
        }
    }
    fn to_list_item(&self, event_tab: &EventTab) -> ListItem<'_> {
        let text = match self {
            Self::Event => {
                let state = event::get_event(event_tab.event.unwrap_or_default()).unwrap_or_default();
                format!("Event ({})",
                event_tab.event.map(|v| v.to_string()).unwrap_or_default())
                    .create_toggle_str(state)
            }
            Self::FightFortissax => {
                "Fight Fortissax".to_string()
            }
            Self::FightEldenBeast => {
                "Fight Elden Beast".to_string()
            }
            Self::UnlockMetyr => {
                "Unlock Metyr".to_string()
            }
            Self::DlcClear => {
                let state = get_dlc_clear().unwrap_or_default();
                "DLC Clear Flag".create_toggle_str(state)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[CommandsItems] = &[
        Self::Event,
        Self::FightFortissax,
        Self::FightEldenBeast,
        Self::UnlockMetyr,
        Self::DlcClear,
    ];
    const NO_DLC_ARRAY: &[CommandsItems] = &[
        Self::Event,
        Self::FightFortissax,
        Self::FightEldenBeast,
    ];
    fn array(dlc: bool) -> &'static [CommandsItems] {
        if !dlc { Self::NO_DLC_ARRAY } else { Self::ARRAY }
    }
    fn list(event_tab: &EventTab, dlc: bool) -> List<'static> {
        let array = Self::array(dlc);
        let items: Vec<ListItem> = array.iter().map(|i| i.to_list_item(event_tab)).collect();
        tabs_list(items, None, &event_tab.tab, COMMANDS_IDX)
    }
}

impl ReviveOptions {
    fn execute(&self, event_tab: &mut EventTab, dlc: bool) {
        match self {
            Self::FirstEncounter => {
                let new_state = !event_tab.first_encounter;
                event_tab.first_encounter = new_state;
                UiState::update_er(|c| { c.revive_first_encounter = new_state; }).ok();
            }
            Self::WarpOnRevive => {
                let new_state = !event_tab.warp;
                event_tab.warp = new_state;
                UiState::update_er(|c| { c.warp_on_revive = new_state; }).ok();
            }
            Self::MassRevive => {
                mass_revive(dlc, event_tab.first_encounter).send_error()
            }
        }
    }
    fn to_list_item(&self, event_tab: &EventTab) -> ListItem<'static> {
        let text = match self {
            Self::FirstEncounter => {
                "First Encounter".create_toggle_str(event_tab.first_encounter)
            }
            Self::WarpOnRevive => {
                "Warp on Revive".create_toggle_str(event_tab.warp)
            }
            Self::MassRevive => {
                "Mass Revive".to_string()
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[ReviveOptions] = &[
        Self::FirstEncounter,
        Self::WarpOnRevive,
        Self::MassRevive
    ];
    fn list(event_tab: &EventTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(event_tab)).collect();
        List::new(items)
            .block(Block::new().borders(Borders::TOP))
            .highlight_style(event_tab.tab.highlight_style(REVIVE_OPTIONS_IDX))
            .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
            .style(event_tab.tab.block_style(REVIVE_OPTIONS_IDX))
    }
}