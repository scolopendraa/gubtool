use crate::{
    ds2::{chr_ctrl::ChrCtrlExt, game_state::{GameStateFlags, get_state_flag, set_state_flag}, player::player_ctrl},
    tui::{
        common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list},
        event::ResultExt,
    },
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem},
};

enum ActionsItems {
}

enum TogglesItems {
    NoDeath,
}

pub enum Stats {
}

const TOGGLES_IDX: usize = 0;
const ACTIONS_IDX: usize = 1;
pub const STATS_IDX: usize = 2;

pub struct PlayerTab {
    tab: TabState,
}

impl PlayerTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[STATS_IDX] = StatefulList::new(0);
        PlayerTab {
            tab: TabState::new(list_states),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let [area_one, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(layout);

        let [area_two, area_three] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .areas(right);

        let layout = [area_one, area_two, area_three];

        frame.render_stateful_widget(
            ActionsItems::list(self),
            layout[ACTIONS_IDX],
            &mut self.tab.get_list_state(ACTIONS_IDX),
        );
        frame.render_stateful_widget(
            TogglesItems::list(self),
            layout[TOGGLES_IDX],
            &mut self.tab.get_list_state(TOGGLES_IDX),
            );
        frame.render_stateful_widget(
            Stats::list(self),
            layout[STATS_IDX],
            &mut self.tab.get_list_state(STATS_IDX),
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_enter(),
            _ => (),
        }
    }

    fn handle_input(&mut self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected].set_input(),
                STATS_IDX => Stats::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }
    fn handle_enter(&mut self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected].execute(),
                TOGGLES_IDX => TogglesItems::ARRAY[selected].execute(),
                STATS_IDX => Stats::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }
}

impl ActionsItems {
    fn execute(&self) {
        match self {
            _ => (),
        }
    }
    fn set_input(&self) {
        match self {
            _ => (),
        }
    }
    fn to_list_item(&self) -> ListItem<'static> {
        let text = match self {
            _ => "",
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &player_tab.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self) {
        match self {
            Self::NoDeath => {
                let new_state = !get_state_flag(GameStateFlags::PlayerNoDeath);
                set_state_flag(GameStateFlags::PlayerNoDeath, new_state).send_error();
                player_ctrl().set_no_death(new_state).ok();
            }
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::NoDeath => {
                let state = get_state_flag(GameStateFlags::PlayerNoDeath);
                "No Death".create_toggle_str(state)
            }
        };
        ListItem::from(text)
    }
    const ARRAY: &[TogglesItems] = &[
        Self::NoDeath,
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &player_tab.tab, TOGGLES_IDX)
    }
}

impl Stats {
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            _ => ""
        };
        ListItem::from(text)
    }
    fn set_input(&self) {

    }

    pub fn set_stat(&self) -> Result<()> {
        match self {
            _ => Ok(()),
        }
    }
    fn increment_stat(&self) -> Result<()> {
        match self {
            _ => Ok(()),
        }
    }
    const ARRAY: &[Stats] = &[
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, Some("Stats"), &player_tab.tab, STATS_IDX)
    }
}
