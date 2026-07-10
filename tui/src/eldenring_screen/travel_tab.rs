use crate::{
    app::App,
    common::{
        block, blockless_list, controls::draw_controls, label_list, stateful_list::StatefulList,
        tab_state::TabState,
    },
    eldenring_screen::GameState,
    event::AnyhowExt,
    input::request_search,
    mutate_app, spawn_task,
    theme::theme,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use eldenring::{
    event,
    resources::{bosses::bosses_array, graces::graces_array},
};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{List, ListItem},
};
use ratatui_themes::Style;

const CONTROLS: &[(&str, &str)] = &[
    ("r", "Revive"),
    ("ctrl-r", "Revive FE"),
];

const BOSSES_IDX: usize = 0;
const GRACES_IDX: usize = 1;

pub struct TravelTab {
    tab: TabState,
}

impl TravelTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 2];
        list_states[BOSSES_IDX] = StatefulList::new(0);
        list_states[GRACES_IDX] = StatefulList::new(0);
        TravelTab {
            tab: TabState::new(list_states),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(layout);

        let bosses_block = block(Some("Bosses"), Some(self.tab.block_style(BOSSES_IDX)))
            .title(self.revive_status_line().right_aligned());
        let bosses_inner = bosses_block.inner(layout[BOSSES_IDX]);
        frame.render_widget(&bosses_block, layout[BOSSES_IDX]);

        let [boss_name, boss_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(30),
            ])
            .areas(bosses_inner);

        let (boss_names, boss_areas) = self.bosses_list();
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
        draw_controls(frame, layout[BOSSES_IDX], CONTROLS);

        let graces_block = block(Some("Graces"), Some(self.tab.block_style(GRACES_IDX)));
        let graces_inner = graces_block.inner(layout[GRACES_IDX]);
        frame.render_widget(&graces_block, layout[GRACES_IDX]);

        let [grace_name, grace_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40),
                Constraint::Max(30),
            ])
            .areas(graces_inner);

        let (grace_names, grace_areas) = self.graces_list();
        frame.render_stateful_widget(
            grace_names,
            grace_name,
            &mut self.tab.get_list_state(GRACES_IDX),
        );
        frame.render_stateful_widget(
            grace_areas,
            grace_area,
            &mut self.tab.get_list_state(GRACES_IDX),
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tab.set_length(BOSSES_IDX, bosses_array(GameState::dlc()).len());
        self.tab.set_length(GRACES_IDX, graces_array(GameState::dlc()).len());

        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => {
                self.handle_select()
            }
            KeyCode::Char('f') => {
                let entries = if self.tab.current_list == BOSSES_IDX {
                    bosses_array(GameState::dlc()).iter()
                        .map(|boss| Utf32String::from(format!("{}|{}", boss.name, boss.main_area)))
                        .collect::<Vec<Utf32String>>()
                } else {
                    graces_array(GameState::dlc()).iter()
                        .map(|grace| Utf32String::from(format!("{}|{}", grace.name, grace.main_area)))
                        .collect::<Vec<Utf32String>>()
                };
                spawn_task! {
                    if let Some(new_idx) = request_search(entries).await {
                        mutate_app!(|app: &mut App| {
                            let tab = &mut app.elden_ring.travel.tab;
                            tab.set_list_selected(tab.current_list, new_idx);
                        });
                    }
                }
            }
            KeyCode::Char('r') => {
                if self.tab.current_list == BOSSES_IDX
                && let Some(selected) = self.tab.get_list_selected(BOSSES_IDX) {
                    let first_encounter = key.modifiers == KeyModifiers::CONTROL;
                    bosses_array(GameState::dlc())[selected]
                        .revive(first_encounter)
                        .send_error()
                }
            }
            _ => ()
        }
    }

    fn handle_select(&self) {
        let Some(selected_idx) = self.tab.get_list_selected(self.tab.current_list) else { return; };
        if self.tab.current_list == BOSSES_IDX {
            spawn_task! {
                bosses_array(GameState::dlc())[selected_idx]
                    .warp()
                    .await
                    .send_error()
            }
        } else if self.tab.current_list == GRACES_IDX {
            graces_array(GameState::dlc())[selected_idx]
                .warp()
                .send_error();
        }
    }

    fn bosses_list(&self) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = bosses_array(GameState::dlc()).iter()
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

    fn graces_list(&self) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = graces_array(GameState::dlc()).iter()
            .map(|grace| (
                    ListItem::from(grace.name),
                    ListItem::from(Line::raw(grace.main_area)).fg(theme().muted)
            ))
            .collect();
        (
            blockless_list(items.0, &self.tab, GRACES_IDX),
            label_list(items.1, &self.tab, GRACES_IDX)
        )
    }

    fn revive_status_line(&self) -> Line<'static> {
        let selected_idx = self.tab.lists_states[BOSSES_IDX].selected().unwrap_or_default();
        let boss = bosses_array(GameState::dlc())[selected_idx];
        let mut style = Style::from(theme().success);
        let text = if !GameState::loaded() {
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