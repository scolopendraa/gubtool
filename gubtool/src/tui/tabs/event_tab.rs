use std::thread;
use crossterm::event::{KeyCode, KeyEvent};
use nucleo_matcher::Utf32String;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use crate::{
    config::{Config, UiState},
    core::event::{self, get_dlc_clear, mass_revive},
    resources::bosses::bosses_array,
    tui::{
        app::App,
        event::Event,
        fuzzy_finder::Picker,
        input_handler::InputField,
        tabs::TabState,
        theme::{self, theme},
        block, blockless_list, label_list, list, ResultExt, StrExt,
    },
};

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
        let mut sizes = vec![0; 3];
        sizes[COMMANDS_IDX] = CommandsItems::ARRAY.len();
        sizes[BOSSES_IDX] = 0;
        sizes[REVIVE_OPTIONS_IDX] = ReviveOptions::ARRAY.len();
        EventTab {
            tab: TabState {
                lists: vec![ListState::default().with_selected(Some(0)); 3],
                list_sizes: sizes,
                ..TabState::default()
            },
            event: None,
            first_encounter: false,
            warp: true,
        }
    }
}

impl CommandsItems {
    fn execute(&self, app: &mut App) {
        let tx = app.sender.clone();
        match self {
            Self::Event => {
                if let Some(event) = app.event.event {
                    let new_state = !event::get_event(event).unwrap_or_default();
                    event::set_event(event, new_state).send_error(tx)
                }
            }
            Self::FightFortissax => {
                event::fight_fortissax().send_error(tx)
            }
            Self::FightEldenBeast => {
                event::fight_elden_beast().send_error(tx)
            }
            Self::UnlockMetyr => {
                event::unlock_metyr().send_error(tx)
            }
            Self::DlcClear => {
                let new_state = !get_dlc_clear().unwrap_or_default();
                event::set_dlc_clear(new_state).send_error(tx)
            }
        }
    }
    fn set_input(&self, app: &mut App) {
        match self {
            Self::Event => app.open_input(InputField::Event),
            _ => (),
        }
    }
    fn to_list_item(&self, app: &App) -> ListItem<'_> {
        let text = match self {
            Self::Event => {
                let state = event::get_event(app.event.event.unwrap_or_default()).unwrap_or_default();
                format!("Event ({})",
                app.event.event.map(|v| v.to_string()).unwrap_or_default())
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
    fn array(app: &App) -> &'static [CommandsItems] {
        if app.attached && !app.game_state.dlc { Self::NO_DLC_ARRAY } else { Self::ARRAY }
    }
    fn list(app: &App) -> List<'static> {
        let array = Self::array(app);
        let items: Vec<ListItem> = array.iter().map(|i| i.to_list_item(app)).collect();
        list(items, None, &app.event.tab, COMMANDS_IDX)
    }
}

impl ReviveOptions {
    fn execute(&self, app: &mut App) {
        match self {
            Self::FirstEncounter => {
                let new_state = !app.event.first_encounter;
                app.event.first_encounter = new_state;
                UiState::update(|c| { c.revive_first_encounter = new_state; }).ok();
            }
            Self::WarpOnRevive => {
                let new_state = !app.event.warp;
                app.event.warp = new_state;
                UiState::update(|c| { c.warp_on_revive = new_state; }).ok();
            }
            Self::MassRevive => {
                mass_revive(app.game_state.dlc, app.event.first_encounter).send_error(app.sender.clone())
            }
        }
    }
    fn to_list_item(&self, app: &App) -> ListItem<'static> {
        let text = match self {
            Self::FirstEncounter => {
                "First Encounter".create_toggle_str(app.event.first_encounter)
            }
            Self::WarpOnRevive => {
                "Warp on Revive".create_toggle_str(app.event.warp)
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
    fn list(app: &App) -> List<'static> {
        let tab = &app.event.tab;
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(app)).collect();
        List::new(items)
            .block(Block::new().borders(Borders::TOP))
            .highlight_style(tab.highlight_style(REVIVE_OPTIONS_IDX))
            .highlight_symbol(theme::HIGHLIGHT_SYMBOL)
            .style(tab.block_style(REVIVE_OPTIONS_IDX))
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

    let block = revive_block(app);
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
        CommandsItems::list(app),
        layout[COMMANDS_IDX],
        &mut app.event.tab.lists[COMMANDS_IDX],
    );

    let [boss_name, boss_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(44),
            Constraint::Max(33),
        ])
        .areas(revive_list);

    let (boss_names, boss_areas) = bosses_list(app);
    frame.render_stateful_widget(
        boss_names,
        boss_name,
        &mut app.event.tab.lists[BOSSES_IDX],
    );
    frame.render_stateful_widget(
        boss_areas,
        boss_area,
        &mut app.event.tab.lists[BOSSES_IDX],
    );
    frame.render_stateful_widget(
        ReviveOptions::list(app),
        revive_options,
        &mut app.event.tab.lists[REVIVE_OPTIONS_IDX],
    );
}

fn revive_block(app: &App) -> Block<'static> {
    let style = if app.event.tab.current_list == BOSSES_IDX {
        app.event.tab.block_style(BOSSES_IDX)
    } else {
        app.event.tab.block_style(REVIVE_OPTIONS_IDX)
    };
    block(Some("Boss Revives"), Some(style))
        .title(revive_status_line(app).right_aligned())
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    app.event.tab.list_sizes[BOSSES_IDX] = bosses_array(app.game_state.dlc).len();
    app.event.tab.handle_keys(key);
    match key.code {
        KeyCode::Char('s') => handle_input(app),
        KeyCode::Enter => handle_select(app),
        KeyCode::Char('f') if matches!(app.event.tab.current_list, BOSSES_IDX | REVIVE_OPTIONS_IDX) => {
            app.set_fuzzy_finder(Box::new(Bosses::new(app.game_state.dlc)));
        }
        _ => (),
    }
}

fn handle_input(app: &mut App) {
    let tab = &app.event.tab;
    let current_list = tab.current_list;
    if let Some(selected_idx) = tab.lists[current_list].selected() {
        match current_list {
            COMMANDS_IDX => CommandsItems::ARRAY[selected_idx].set_input(app),
            _ => (),
        }
    }
}

fn handle_select(app: &mut App) {
    let tab = &mut app.event.tab;
    let current_list = tab.current_list;
    let Some(selected_idx) = tab.lists[current_list].selected() else { return };

    if current_list == COMMANDS_IDX {
        CommandsItems::array(app)[selected_idx].execute(app)
    }
    if current_list == BOSSES_IDX {
        let first_encounter = app.event.first_encounter;
        let warp = app.event.warp;
        let dlc = app.game_state.dlc;
        let tx = app.sender.clone();
        thread::spawn(move || {
            if let Err(err) = bosses_array(dlc)[selected_idx].revive(first_encounter, warp) {
                tx.send(Event::Error(err.to_string())).ok();
            }
        });
    }
    if current_list == REVIVE_OPTIONS_IDX {
        ReviveOptions::ARRAY[selected_idx].execute(app)
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
        blockless_list(items.0, &app.event.tab, BOSSES_IDX),
        label_list(items.1, &app.event.tab, BOSSES_IDX)
    )
}

fn revive_status_line(app: &App) -> Line<'static> {
    let selected_idx = app.event.tab.lists[BOSSES_IDX].selected().unwrap_or_default();
    let boss = bosses_array(app.game_state.dlc)[selected_idx];
    let mut style = Style::from(theme().success);
    let text = if !app.game_state.loaded {
        "".to_string()
    } else {
        boss.revive_status().to_string()
    };
    if app.event.tab.current_list != BOSSES_IDX {
        style = Style::from(theme().fg)
    } else if text == event::DEAD {
        style = Style::from(theme().error)
    }
    Line::from(text)
        .style(style)
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
        *app.event.tab.lists[BOSSES_IDX].selected_mut() = Some(idx);
    }
}
