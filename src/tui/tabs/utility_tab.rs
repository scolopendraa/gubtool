use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame, symbols,
};
use crate::{
    config::{Config, Preferences},
    core::{
        game_state::{self, GameStateFlags},
        utility,
    },
    resources::talk_commands::{MENUS, SHOPS, shops_array},
    tui::{
        app::App,
        input_handler::InputField,
        tabs::TabState,
        block, list, ResultExt, StrExt, theme, blockless_list,
    }
};

enum OptionsItems {
    FpsCap,
    GameSpeed,
    Quitout,
    RemoveLogos,
    ToggleMusic,
    ShowAllMaps,
    ShowAllGraces,
    ClearCount,
    TriggerNewGameCycle,
    StutterFix,
    FreezeWorld,
    DisableAreaTitleCards,
    DrawHitboxesA,
}

enum PreferencesItems {
    FpsCap,
    NoDeath,
    NoDamage,
    SetRfbsOnLoad,
    InfinitePoise,
    MuteMusic,
    RemoveLogos,
    DisableAreaTitleCards,
    StutterFix,
}

const OPTIONS_IDX: usize = 0;
pub const PREFERENCES_IDX: usize = 1;
const MENUS_IDX: usize = 2;
const SHOPS_IDX: usize = 3;

pub struct UtilityTab {
    pub tab: TabState,
    preferences: Preferences,
    menu_shop_idx: usize,
}

impl UtilityTab {
    pub fn new() -> Self {
        let mut sizes = vec![0; 4];
        sizes[OPTIONS_IDX] = OptionsItems::ARRAY.len();
        sizes[PREFERENCES_IDX] = PreferencesItems::ARRAY.len();
        sizes[MENUS_IDX] = MENUS.len();
        sizes[SHOPS_IDX] = SHOPS.len();
        UtilityTab {
            tab: TabState {
                lists: vec![ListState::default().with_selected(Some(0)); 4],
                list_sizes: sizes,
                ..TabState::default()
            },
            preferences: Preferences::read().unwrap_or_default(),
            menu_shop_idx: 0,
        }
    }
}

impl OptionsItems {
    fn execute(&self, app: &mut App) {
        let tx = app.sender.clone();
        match self {
            Self::FpsCap => {
                app.open_input(InputField::Fps)
            }
            Self::GameSpeed => {
                app.open_input(InputField::GameSpeed)
            }
            Self::Quitout => {
                utility::quitout().send_error(tx)
            }
            Self::ToggleMusic => {
                let new_state = !utility::is_music_muted().unwrap_or_default();
                utility::mute_music(new_state).send_error(tx)
            }
            Self::RemoveLogos => {
                let new_state = !utility::is_logo_patch().unwrap_or_default();
                utility::set_logo_patch(new_state).send_error(tx)
            }
            Self::ShowAllMaps => {
                let new_state = !utility::is_show_all_maps_on().unwrap_or_default();
                utility::show_all_maps(new_state).send_error(tx)
            }
            Self::ShowAllGraces => {
                let new_state = !utility::is_show_all_graces_on().unwrap_or_default();
                utility::show_all_graces(new_state).send_error(tx)
            }
            Self::ClearCount => {
                app.open_input(InputField::NewGameCycle)
            }
            Self::TriggerNewGameCycle => {
                utility::trigger_new_game().send_error(tx)
            }
            Self::StutterFix => {
                let new_state = !utility::is_stutter_fix_on().unwrap_or_default();
                utility::set_stutter_fix(new_state).send_error(tx)
            }
            Self::FreezeWorld => {
                let new_state = !utility::is_freeze_world_on().unwrap_or_default();
                utility::set_freeze_world(new_state).send_error(tx)
            }
            Self::DisableAreaTitleCards => {
                let new_state = !game_state::get_state_flag(GameStateFlags::TitleCards).unwrap_or_default();
                game_state::set_state_flag(GameStateFlags::TitleCards, new_state).send_error(tx);
            }
            Self::DrawHitboxesA => {
                let new_state = !utility::is_hitboxes(false).unwrap_or_default();
                utility::draw_hitboxes(new_state, false).send_error(tx)
            }
        }
    }
    fn set_input(&self, app: &mut App) {
        match self {
            Self::FpsCap => app.open_input(InputField::Fps),
            Self::GameSpeed => app.open_input(InputField::GameSpeed),
            Self::ClearCount => app.open_input(InputField::NewGameCycle),
            _ => (),
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::FpsCap => {
                format!("FPS Cap: {}",
                    utility::get_fps_cap().unwrap_or_default())
            }
            Self::GameSpeed => {
                format!("Game Speed: {}",
                    utility::get_game_speed().unwrap_or_default())
            }
            Self::Quitout => {
                "Quitout".to_string()
            }
            Self::ToggleMusic => {
                let state = utility::is_music_muted().unwrap_or_default();
                "Mute Music".create_toggle_str(state)
            }
            Self::RemoveLogos => {
                let state = utility::is_logo_patch().unwrap_or_default();
                "Remove Logos".create_toggle_str(state)
            }
            Self::ShowAllMaps => {
                let state = utility::is_show_all_maps_on().unwrap_or_default();
                "Show All Maps".create_toggle_str(state)
            }
            Self::ShowAllGraces => {
                let state = utility::is_show_all_graces_on().unwrap_or_default();
                "Show All Graces".create_toggle_str(state)
            }
            Self::ClearCount => {
                format!("ClearCount: {}",
                    utility::get_ng_cycle().unwrap_or_default())
            }
            Self::TriggerNewGameCycle => {
                "Trigger New Game Cycle".to_string()
            }
            Self::StutterFix => {
                let state = utility::is_stutter_fix_on().unwrap_or_default();
                "Stutter Fix".create_toggle_str(state)
            }
            Self::FreezeWorld => {
                let state = utility::is_freeze_world_on().unwrap_or_default();
                "Freeze World".create_toggle_str(state)
            }
            Self::DisableAreaTitleCards => {
                let state = game_state::get_state_flag(GameStateFlags::TitleCards).unwrap_or_default();
                "Disable Area Title Cards".create_toggle_str(state)
            }
            Self::DrawHitboxesA => {
                let state = utility::is_hitboxes(false).unwrap_or_default();
                "Draw Hitboxes".create_toggle_str(state)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[OptionsItems] = &[
        Self::FpsCap,
        Self::GameSpeed,
        Self::FreezeWorld,
        Self::ToggleMusic,
        Self::RemoveLogos,
        Self::DisableAreaTitleCards,
        Self::DrawHitboxesA,
        Self::ShowAllGraces,
        Self::ShowAllMaps,
        Self::Quitout,
        Self::ClearCount,
        Self::TriggerNewGameCycle,
        Self::StutterFix,
    ];
    fn list(app: &App) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        list(items, None, &app.utility.tab, OPTIONS_IDX)
    }
}

impl PreferencesItems {
    fn execute(&self, app: &mut App) {
        let tx = app.sender.clone();
        match self {
            Self::FpsCap => {
                app.open_input(InputField::ConfigFps)
            }
            Self::NoDeath => {
                Preferences::update(|c| c.no_death = !c.no_death).send_error(tx)
            }
            Self::NoDamage => {
                Preferences::update(|c| c.no_damage = !c.no_damage).send_error(tx)
            }
            Self::SetRfbsOnLoad => {
                Preferences::update(|c| c.rfbs_on_load = !c.rfbs_on_load).send_error(tx)
            }
            Self::InfinitePoise => {
                Preferences::update(|c| c.infinite_poise = !c.infinite_poise).send_error(tx)
            }
            Self::MuteMusic => {
                Preferences::update(|c| c.mute_music = !c.mute_music).send_error(tx)
            }
            Self::RemoveLogos => {
                Preferences::update(|c| c.remove_logo = !c.remove_logo).send_error(tx)
            }
            Self::StutterFix => {
                Preferences::update(|c| c.stutter_fix = !c.stutter_fix).send_error(tx)
            }
            Self::DisableAreaTitleCards => {
                Preferences::update(|c| c.disable_area_target_cards = !c.disable_area_target_cards).send_error(tx)
            }
        }
    }
    fn set_input(&self, app: &mut App) {
        match self {
            Self::FpsCap => app.open_input(InputField::ConfigFps),
            _ => (),
        }
    }
    fn to_list_item(&self, preferences: &Preferences) -> ListItem<'_> {
        let text = match self {
            Self::FpsCap => {
                format!("FPS Cap: {}", preferences.fps.map_or("".to_string(), |v| v.to_string()))
            }
            Self::NoDeath => {
                "No Death".create_toggle_str(preferences.no_death)
            }
            Self::NoDamage => {
                "No Damage".create_toggle_str(preferences.no_damage)
            }
            Self::SetRfbsOnLoad => {
                "Set RFBS on load".create_toggle_str(preferences.rfbs_on_load)
            }
            Self::InfinitePoise => {
                "Infinite Poise".create_toggle_str(preferences.infinite_poise)
            }
            Self::MuteMusic => {
                "Mute Music".create_toggle_str(preferences.mute_music)
            }
            Self::RemoveLogos => {
                "Remove Logos".create_toggle_str(preferences.remove_logo)
            }
            Self::StutterFix => {
                "Stutter Fix".create_toggle_str(preferences.stutter_fix)
            }
            Self::DisableAreaTitleCards => {
                "Disable Area Title Cards".create_toggle_str(preferences.disable_area_target_cards)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[PreferencesItems] = &[
        Self::FpsCap,
        Self::NoDeath,
        Self::NoDamage,
        Self::SetRfbsOnLoad,
        Self::InfinitePoise,
        Self::MuteMusic,
        Self::RemoveLogos,
        Self::DisableAreaTitleCards,
        Self::StutterFix,
    ];
    fn list(app: &App) -> List<'static> {
        let tab = &app.utility.tab;
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(&app.utility.preferences)).collect();
        blockless_list(items, tab, PREFERENCES_IDX)
    }
}

pub fn draw(frame: &mut Frame, app: &mut App, layout: Rect) {
    app.utility.preferences = Preferences::read().unwrap_or_default();

    let [area_one, right_area] = Layout::default()
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
        .areas(right_area);

    let layout = [area_one, area_two, area_three];

    let block = block(Some("Preferences"), Some(app.utility.tab.block_style(PREFERENCES_IDX)));
    frame.render_widget(&block, layout[PREFERENCES_IDX]);
    let inner = block.inner(layout[PREFERENCES_IDX]);
    let paragraph_len: u16 = 78;
    let paragraph_vertical_length = if inner.width == 0 { 0 } else { paragraph_len.div_ceil(inner.width) };

    let [prefs_paragraph, prefs_list] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(paragraph_vertical_length),
            Constraint::Fill(1),
        ])
        .areas(inner);

    frame.render_stateful_widget(
        OptionsItems::list(app),
        layout[OPTIONS_IDX],
        &mut app.utility.tab.lists[OPTIONS_IDX],
    );
    frame.render_widget(preferences_paragraph(), prefs_paragraph);
    frame.render_stateful_widget(
        PreferencesItems::list(app),
        prefs_list,
        &mut app.utility.tab.lists[PREFERENCES_IDX],
    );

    if app.utility.menu_shop_idx == 1 {
        frame.render_stateful_widget(shops_list(app), layout[MENUS_IDX], &mut app.utility.tab.lists[SHOPS_IDX]);
    } else {
        frame.render_stateful_widget(menus_list(app), layout[MENUS_IDX], &mut app.utility.tab.lists[MENUS_IDX]);
    }
    frame.render_widget(menu_shop_tab(app), layout[MENUS_IDX]);
}

fn menu_shop_tab(app: &App) -> Tabs<'static> {
    Tabs::new(vec!["Menus", "Shops"])
        .highlight_style(
            if app.utility.menu_shop_idx == 0 {
                app.utility.tab.highlight_style(MENUS_IDX)
            } else {
                app.utility.tab.highlight_style(SHOPS_IDX)
            })
        .select(app.utility.menu_shop_idx)
        .divider(symbols::DOT)
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    let tab = &mut app.utility.tab;

    tab.handle_keys(key);

    match tab.current_list {
        MENUS_IDX if app.utility.menu_shop_idx == 1 => tab.current_list = SHOPS_IDX,
        SHOPS_IDX if app.utility.menu_shop_idx == 0 => tab.current_list = MENUS_IDX,
        _ => (),
    }
    match tab.current_list {
        SHOPS_IDX => match (key.code, key.modifiers) {
            (KeyCode::Char('h'), _) => {
                tab.current_list = MENUS_IDX;
                app.utility.menu_shop_idx = 0
            }
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                tab.current_list = PREFERENCES_IDX
            }
            _ => (),
        },
        MENUS_IDX => {
            if key.code == KeyCode::Char('l') {
                tab.current_list = SHOPS_IDX;
                app.utility.menu_shop_idx = 1
            }
        }
        _ => (),
    }
    match key.code {
        KeyCode::Char('s') => handle_input(app),
        KeyCode::Enter => handle_enter(app),
        _ => (),
    }
}

fn handle_input(app: &mut App) {
    let tab = &app.utility.tab;
    let current_list = tab.current_list;
    if let Some(selected_idx) = tab.lists[current_list].selected() {
        match current_list {
            OPTIONS_IDX => OptionsItems::ARRAY[selected_idx].set_input(app),
            PREFERENCES_IDX => PreferencesItems::ARRAY[selected_idx].set_input(app),
            _ => (),
        }
    }
}

fn handle_enter(app: &mut App) {
    let tab = &app.utility.tab;
    let current_list = tab.current_list;
    if let Some(selected_idx) = tab.lists[current_list].selected() {
        let tx = app.sender.clone();
        match current_list {
            OPTIONS_IDX => OptionsItems::ARRAY[selected_idx].execute(app),
            PREFERENCES_IDX => PreferencesItems::ARRAY[selected_idx].execute(app),
            MENUS_IDX => MENUS[selected_idx].execute().send_error(tx),
            SHOPS_IDX => shops_array(app.game_state.dlc)[selected_idx].execute().send_error(tx),
            _ => (),
        }
    }
}

fn preferences_paragraph() -> Paragraph<'static> {
    Paragraph::new("Values in this list will be initialized when the tool attaches to the game")
        .style(theme().muted)
        .wrap(Wrap { trim: true })
}

fn menus_list(app: &App) -> List<'static> {
    let items: Vec<ListItem> = MENUS.iter().map(|menu| ListItem::new(menu.name)).collect();
    list(items, None, &app.utility.tab, MENUS_IDX)
}

fn shops_list(app: &App) -> List<'static> {
    let items: Vec<ListItem> = shops_array(app.game_state.dlc).iter().map(|shop| ListItem::from(shop.name)).collect();
    list(items, None, &app.utility.tab, SHOPS_IDX)
}
