use std::thread;
use crossterm::event::{KeyCode, KeyEvent};
use nucleo_matcher::Utf32String;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{List, ListItem},
    Frame,
};
use crate::{
    resources::{
        bosses::bosses_array,
        graces::graces_array
    },
    tui::{
        app::App,
        event::Event,
        fuzzy_finder::Picker,
        tabs::TabState,
        block, blockless_list, label_list, theme,
    },
};

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
}

pub fn draw(frame: &mut Frame, app: &mut App, layout: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(layout);

    let bosses_block = block(Some("Bosses"), Some(app.travel.tab.block_style(BOSSES_IDX)));
    let bosses_inner = bosses_block.inner(layout[BOSSES_IDX]);
    frame.render_widget(&bosses_block, layout[BOSSES_IDX]);

    let [boss_name, boss_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(44),
            Constraint::Max(33),
        ])
        .areas(bosses_inner);

    let (boss_names, boss_areas) = bosses_list(app);
    frame.render_stateful_widget(
        boss_names,
        boss_name,
        &mut app.travel.tab.lists[BOSSES_IDX],
    );
    frame.render_stateful_widget(
        boss_areas,
        boss_area,
        &mut app.travel.tab.lists[BOSSES_IDX],
    );

    let graces_block = block(Some("Graces"), Some(app.travel.tab.block_style(GRACES_IDX)));
    let graces_inner = graces_block.inner(layout[GRACES_IDX]);
    frame.render_widget(&graces_block, layout[GRACES_IDX]);

    let [grace_name, grace_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(42),
            Constraint::Max(33),
        ])
        .areas(graces_inner);

    let (grace_names, grace_areas) = graces_list(app);
    frame.render_stateful_widget(
        grace_names,
        grace_name,
        &mut app.travel.tab.lists[GRACES_IDX],
    );
    frame.render_stateful_widget(
        grace_areas,
        grace_area,
        &mut app.travel.tab.lists[GRACES_IDX],
    );
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let dlc_available = app.game_state.dlc;
    let tab = &mut app.travel.tab;
    tab.list_sizes[BOSSES_IDX] = bosses_array(dlc_available).len();
    tab.list_sizes[GRACES_IDX] = graces_array(dlc_available).len();

    tab.handle_keys(key);
    match key.code {
        KeyCode::Enter => {
            handle_select(app)
        }
        KeyCode::Char('f') => {
            if tab.current_list == BOSSES_IDX {
                app.set_fuzzy_finder(Box::new(Bosses::new(app.game_state.dlc)));
            } else {
                app.set_fuzzy_finder(Box::new(Graces::new(app.game_state.dlc)));
            }
        }
        _ => ()
    }
}

fn handle_select(app: &App) {
    let tab = &app.travel.tab;
    let current_list = tab.current_list;
    let Some(selected_idx) = tab.lists[current_list].selected() else { return; };
    if current_list == BOSSES_IDX {
        let dlc = app.game_state.dlc;
        let tx = app.sender.clone();
        thread::spawn(move || {
            if let Err(err) = bosses_array(dlc)[selected_idx].warp() {
                tx.send(Event::Error(err.to_string())).ok();
            }
        });
    }
    if current_list == GRACES_IDX {
        let dlc = app.game_state.dlc;
        let tx = app.sender.clone();
        thread::spawn(move || {
            if let Err(err) = graces_array(dlc)[selected_idx].warp() {
                tx.send(Event::Error(err.to_string())).ok();
            }
        });
    }
}

fn bosses_list(app: &App) -> (List<'static>, List<'static>) {
    let items: (Vec<ListItem>, Vec<ListItem>) = bosses_array(app.game_state.dlc).iter()
        .map(|boss| (
                ListItem::from(boss.name),
                ListItem::from(Line::raw(boss.main_area)).fg(theme().muted)
        ))
        .collect();
    (
        blockless_list(items.0, &app.travel.tab, BOSSES_IDX),
        label_list(items.1, &app.travel.tab, BOSSES_IDX)
    )
}

fn graces_list(app: &App) -> (List<'static>, List<'static>) {
    let items: (Vec<ListItem>, Vec<ListItem>) = graces_array(app.game_state.dlc).iter()
        .map(|grace| (
                ListItem::from(grace.name),
                ListItem::from(Line::raw(grace.main_area)).fg(theme().muted)
        ))
        .collect();
    (
        blockless_list(items.0, &app.travel.tab, GRACES_IDX),
        label_list(items.1, &app.travel.tab, GRACES_IDX)
    )
}

struct Bosses {
    pub entries: Vec<Utf32String>,
}

impl Bosses {
    pub fn new(dlc: bool) -> Self {
        let entries: Vec<Utf32String> = bosses_array(dlc).iter()
            .map(|boss| Utf32String::from(format!("{}|{}", boss.name, boss.main_area)))
            .collect();
        Self { entries }
    }
}

impl Picker for Bosses {
    fn items(&self) -> Vec<Utf32String> {
        self.entries.to_vec()
    }
    fn jump(&self, idx: usize, app: &mut App) {
        *app.travel.tab.lists[BOSSES_IDX].selected_mut() = Some(idx);
    }
}

struct Graces {
    pub entries: Vec<Utf32String>,
}

impl Graces {
    pub fn new(dlc: bool) -> Self {
        let entries: Vec<Utf32String> = graces_array(dlc).iter()
            .map(|grace| Utf32String::from(format!("{}|{}", grace.name, grace.main_area)))
            .collect();
        Self { entries }
    }
}

impl Picker for Graces {
    fn items(&self) -> Vec<Utf32String> {
        self.entries.to_vec()
    }
    fn jump(&self, idx: usize, app: &mut App) {
        *app.travel.tab.lists[GRACES_IDX].selected_mut() = Some(idx);
    }
}
