use crate::{
    ds2::resources::warps,
    tui::{
        app::App,
        common::{
            block, blockless_list, label_list, stateful_list::StatefulList, tab_state::TabState,
            tabs_list,
        },
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
const BONFIRES_IDX: usize = 1;

pub struct TravelTab {
    tab: TabState,
}

impl TravelTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 2];
        list_states[BOSSES_IDX] = StatefulList::new(warps::BOSS_WARPS.len());
        list_states[BONFIRES_IDX] = StatefulList::new(warps::BONFIRES.len());
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

        frame.render_stateful_widget(
            self.bosses_list(),
            layout[BOSSES_IDX],
            &mut self.tab.get_list_state(BOSSES_IDX),
        );

        let bonfires_block = block(Some("Bonfires"), Some(self.tab.block_style(BONFIRES_IDX)));
        let bonfires_inner = bonfires_block.inner(layout[BONFIRES_IDX]);
        frame.render_widget(&bonfires_block, layout[BONFIRES_IDX]);

        let [bonfire_name, bonfire_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(42),
                Constraint::Max(33),
            ])
            .areas(bonfires_inner);

        let (bonfire_names, bonfire_areas) = self.bonfires_list();
        frame.render_stateful_widget(
            bonfire_names,
            bonfire_name,
            &mut self.tab.get_list_state(BONFIRES_IDX),
        );
        frame.render_stateful_widget(
            bonfire_areas,
            bonfire_area,
            &mut self.tab.get_list_state(BONFIRES_IDX),
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent) {
        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Enter => {
                self.handle_select()
            }
            KeyCode::Char('f') => {
                if self.tab.current_list == BOSSES_IDX {
                    let list = warps::BOSS_WARPS.iter()
                        .map(|boss| Utf32String::from(format!("{}", boss.name)))
                        .collect();

                    let function = |app: &mut App| {
                        app.dark_souls_2.travel.tab.set_list_selected(
                            BOSSES_IDX,
                            app.fuzzy_finder.selected_idx().unwrap()
                        )
                    };
                    send_event(Event::Search((list, function)))
                } else {
                    let list = warps::BONFIRES.iter()
                        .map(|bonfire| Utf32String::from(format!("{}|{}", bonfire.name, bonfire.main_area)))
                        .collect();

                    let function = |app: &mut App| {
                        app.dark_souls_2.travel.tab.set_list_selected(
                            BONFIRES_IDX,
                            app.fuzzy_finder.selected_idx().unwrap()
                        )
                    };
                    send_event(Event::Search((list, function)))
                }
            }
            _ => ()
        }
    }

    fn handle_select(&self) {
        let Some(selected) = self.tab.get_list_selected(self.tab.current_list) else { return };
        if self.tab.current_list == BOSSES_IDX {
            thread::spawn(move || {
                warps::BOSS_WARPS[selected].warp().send_error()
            });
        } else if self.tab.current_list == BONFIRES_IDX {
            thread::spawn(move || {
                warps::BONFIRES[selected].warp().send_error()
            });
        }
    }

    fn bosses_list(&self) -> List<'static> {
        let items: Vec<ListItem> = warps::BOSS_WARPS.iter()
            .map(|boss| ListItem::from(boss.name)).collect();

        tabs_list(items, Some("Bosses"), &self.tab, BOSSES_IDX)
    }

    fn bonfires_list(&self) -> (List<'static>, List<'static>) {
        let items: (Vec<ListItem>, Vec<ListItem>) = warps::BONFIRES.iter()
            .map(|bonfire| (
                    ListItem::from(bonfire.name),
                    ListItem::from(Line::raw(bonfire.main_area)).fg(theme().muted)
            ))
            .collect();
        (
            blockless_list(items.0, &self.tab, BONFIRES_IDX),
            label_list(items.1, &self.tab, BONFIRES_IDX)
        )
    }
}