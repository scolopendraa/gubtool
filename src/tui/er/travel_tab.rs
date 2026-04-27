use crate::{
    er::resources::{bosses::bosses_array, graces::graces_array},
    tui::{
        app::App,
        common::{block, blockless_list, label_list, tab_state::TabState},
        er::ErInfo,
        event::{Event, ResultExt, send_event},
        theme::theme,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{List, ListItem},
};
use std::thread;

const BOSSES_IDX: usize = 0;
const GRACES_IDX: usize = 1;

pub struct TravelTab {
    tab: TabState,
}

impl TravelTab {
    pub fn new() -> Self {
        TravelTab {
            tab: TabState {
                list_sizes: vec![0, 0],
                ..TabState::default()
            },
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

        let bosses_block = block(Some("Bosses"), Some(self.tab.block_style(BOSSES_IDX)));
        let bosses_inner = bosses_block.inner(layout[BOSSES_IDX]);
        frame.render_widget(&bosses_block, layout[BOSSES_IDX]);

        let [boss_name, boss_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(44),
                Constraint::Max(33),
            ])
            .areas(bosses_inner);

        let (boss_names, boss_areas) = self.bosses_list(er.dlc);
        frame.render_stateful_widget(
            boss_names,
            boss_name,
            &mut self.tab.lists[BOSSES_IDX],
        );
        frame.render_stateful_widget(
            boss_areas,
            boss_area,
            &mut self.tab.lists[BOSSES_IDX],
        );

        let graces_block = block(Some("Graces"), Some(self.tab.block_style(GRACES_IDX)));
        let graces_inner = graces_block.inner(layout[GRACES_IDX]);
        frame.render_widget(&graces_block, layout[GRACES_IDX]);

        let [grace_name, grace_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(42),
                Constraint::Max(33),
            ])
            .areas(graces_inner);

        let (grace_names, grace_areas) = self.graces_list(er.dlc);
        frame.render_stateful_widget(
            grace_names,
            grace_name,
            &mut self.tab.lists[GRACES_IDX],
        );
        frame.render_stateful_widget(
            grace_areas,
            grace_area,
            &mut self.tab.lists[GRACES_IDX],
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent, er: &ErInfo) {
        self.tab.list_sizes[BOSSES_IDX] = bosses_array(er.dlc).len();
        self.tab.list_sizes[GRACES_IDX] = graces_array(er.dlc).len();

        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Enter => {
                self.handle_select(er.dlc)
            }
            KeyCode::Char('f') => {
                if self.tab.current_list == BOSSES_IDX {
                    let list = bosses_array(er.dlc).iter()
                        .map(|boss| Utf32String::from(format!("{}|{}", boss.name, boss.main_area)))
                        .collect();

                    let function = |app: &mut App| {
                        *app.elden_ring.travel.tab.lists[BOSSES_IDX].selected_mut() =
                        Some(app.fuzzy_finder.selected_idx().unwrap());
                    };
                    send_event(Event::Search((list, function)))
                } else {
                    let list = graces_array(er.dlc).iter()
                        .map(|grace| Utf32String::from(format!("{}|{}", grace.name, grace.main_area)))
                        .collect();

                    let function = |app: &mut App| {
                        *app.elden_ring.travel.tab.lists[GRACES_IDX].selected_mut() =
                        Some(app.fuzzy_finder.selected_idx().unwrap());
                    };
                    send_event(Event::Search((list, function)))
                }
            }
            _ => ()
        }
    }

    fn handle_select(&self, dlc: bool) {
        let Some(selected_idx) = self.tab.lists[self.tab.current_list].selected() else { return; };
        if self.tab.current_list == BOSSES_IDX {
            thread::spawn(move || {
                bosses_array(dlc)[selected_idx].warp().send_error()
            });
        } else if self.tab.current_list == GRACES_IDX {
            thread::spawn(move || {
                graces_array(dlc)[selected_idx].warp().send_error();
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

    fn graces_list(&self, dlc: bool) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = graces_array(dlc).iter()
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
}