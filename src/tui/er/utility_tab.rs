use crate::{
    config::{
        Config,
        preferences::{ErPreferences, Preferences},
    },
    er::{
        game_state::{self, GameStateFlags},
        resources::talk_commands::{MENUS, shops_array},
        utility,
    },
    send_input_event,
    tui::{
        common::{StrExt, block, blockless_list, stateful_list::StatefulList, tab_state::TabState, tabs_list},
        er::ErInfo,
        event::ResultExt,
        theme::theme,
    },
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    symbols,
    widgets::{List, ListItem, Paragraph, Tabs, Wrap},
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
        let mut list_states = vec![StatefulList::new(0); 4];
        list_states[OPTIONS_IDX] = StatefulList::new(OptionsItems::ARRAY.len());
        list_states[PREFERENCES_IDX] = StatefulList::new(PreferencesItems::ARRAY.len());
        list_states[MENUS_IDX] = StatefulList::new(MENUS.len());
        list_states[SHOPS_IDX] = StatefulList::new(0);
        UtilityTab {
            tab: TabState::new(list_states),
            preferences: Preferences::read().unwrap_or_default(),
            menu_shop_idx: 0,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect, er: &ErInfo) {
        self.preferences = Preferences::read().unwrap_or_default();

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

        let block = block(Some("Preferences"), Some(self.tab.block_style(PREFERENCES_IDX)));
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
            OptionsItems::list(self),
            layout[OPTIONS_IDX],
            &mut self.tab.get_list_state(OPTIONS_IDX),
        );
        frame.render_widget(Self::preferences_paragraph(), prefs_paragraph);
        frame.render_stateful_widget(
            PreferencesItems::list(self),
            prefs_list,
            &mut self.tab.get_list_state(PREFERENCES_IDX),
        );

        if self.menu_shop_idx == 1 {
            frame.render_stateful_widget(
                self.shops_list(er.dlc),
                layout[MENUS_IDX],
                &mut self.tab.get_list_state(SHOPS_IDX),
            );
        } else {
            frame.render_stateful_widget(
                self.menus_list(),
                layout[MENUS_IDX],
                &mut self.tab.get_list_state(MENUS_IDX),
            );
        }
        frame.render_widget(self.menu_shop_tab(), layout[MENUS_IDX]);
    }

    fn menu_shop_tab(&self) -> Tabs<'static> {
        Tabs::new(vec!["Menus", "Shops"])
            .highlight_style(
                if self.menu_shop_idx == 0 {
                    self.tab.block_style(MENUS_IDX).fg(theme().secondary)
                } else {
                    self.tab.block_style(SHOPS_IDX).fg(theme().secondary)
                })
            .select(self.menu_shop_idx)
            .divider(symbols::DOT)
    }

    pub fn handle_keys(&mut self, key: KeyEvent, er: &ErInfo) {
        if self.tab.current_list == SHOPS_IDX {
            self.tab.set_length(SHOPS_IDX, shops_array(er.dlc).len())
        }
        self.tab.handle_keys(key);
        match self.tab.current_list {
            MENUS_IDX if self.menu_shop_idx == 1 => self.tab.current_list = SHOPS_IDX,
            SHOPS_IDX if self.menu_shop_idx == 0 => self.tab.current_list = MENUS_IDX,
            _ => (),
        }
        match self.tab.current_list {
            SHOPS_IDX => match (key.code, key.modifiers) {
                (KeyCode::Char('h'), _) => {
                    self.tab.current_list = MENUS_IDX;
                    self.menu_shop_idx = 0
                }
                (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                    self.tab.current_list = PREFERENCES_IDX
                }
                _ => (),
            },
            MENUS_IDX => {
                if key.code == KeyCode::Char('l') {
                    self.tab.current_list = SHOPS_IDX;
                    self.menu_shop_idx = 1
                }
            }
            _ => (),
        }
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_enter(er.dlc),
            _ => (),
        }
    }

    fn handle_input(&self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                OPTIONS_IDX => OptionsItems::ARRAY[selected].set_input(),
                PREFERENCES_IDX => PreferencesItems::ARRAY[selected].set_input(),
                _ => (),
            }
        }
    }

    fn handle_enter(&self, dlc: bool) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                OPTIONS_IDX => OptionsItems::ARRAY[selected].execute(),
                PREFERENCES_IDX => PreferencesItems::ARRAY[selected].execute(),
                MENUS_IDX => MENUS[selected].execute().send_error(),
                SHOPS_IDX => shops_array(dlc)[selected].execute().send_error(),
                _ => (),
            }
        }
    }

    fn preferences_paragraph() -> Paragraph<'static> {
        Paragraph::new("Values in this list will be initialized when the tool attaches to the game")
            .style(theme().muted)
            .wrap(Wrap { trim: true })
    }

    fn menus_list(&self) -> List<'static> {
        let items: Vec<ListItem> = MENUS.iter().map(|menu| ListItem::new(menu.name)).collect();
        tabs_list(items, None, &self.tab, MENUS_IDX)
    }

    fn shops_list(&self, dlc: bool) -> List<'static> {
        let items: Vec<ListItem> = shops_array(dlc).iter().map(|shop| ListItem::from(shop.name)).collect();
        tabs_list(items, None, &self.tab, SHOPS_IDX)
    }
}

impl OptionsItems {
    fn execute(&self) {
        match self {
            Self::FpsCap => {
                send_input_event!(text, _app, {
                    if let Ok(v) = text.parse() {
                        utility::set_fps_cap(v).send_error()
                    }
                })
            }
            Self::GameSpeed => {
                send_input_event!(text, _app, {
                    if let Ok(v) = text.parse() {
                        utility::set_game_speed(v).send_error()
                    }
                })
            }
            Self::Quitout => {
                utility::quitout().send_error()
            }
            Self::ToggleMusic => {
                let new_state = !utility::is_music_muted().unwrap_or_default();
                utility::mute_music(new_state).send_error()
            }
            Self::RemoveLogos => {
                let new_state = !utility::is_logo_patch().unwrap_or_default();
                utility::set_logo_patch(new_state).send_error()
            }
            Self::ShowAllMaps => {
                let new_state = !utility::is_show_all_maps_on().unwrap_or_default();
                utility::show_all_maps(new_state).send_error()
            }
            Self::ShowAllGraces => {
                let new_state = !utility::is_show_all_graces_on().unwrap_or_default();
                utility::show_all_graces(new_state).send_error()
            }
            Self::ClearCount => {
                send_input_event!(text, _app, {
                    if let Ok(v) = text.parse() {
                        utility::set_ng_cycle(v).send_error()
                    }
                })
            }
            Self::TriggerNewGameCycle => {
                utility::trigger_new_game().send_error()
            }
            Self::StutterFix => {
                let new_state = !utility::is_stutter_fix_on().unwrap_or_default();
                utility::set_stutter_fix(new_state).send_error()
            }
            Self::FreezeWorld => {
                let new_state = !utility::is_freeze_world_on().unwrap_or_default();
                utility::set_freeze_world(new_state).send_error()
            }
            Self::DisableAreaTitleCards => {
                let new_state = !game_state::get_state_flag(GameStateFlags::TitleCards);
                game_state::set_state_flag(GameStateFlags::TitleCards, new_state).send_error();
            }
            Self::DrawHitboxesA => {
                let new_state = !utility::is_hitboxes(false).unwrap_or_default();
                utility::draw_hitboxes(new_state, false).send_error()
            }
        }
    }
    fn set_input(&self) {
        match self {
            Self::FpsCap | Self::GameSpeed | Self::ClearCount => {
                self.execute()
            },
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
                let state = game_state::get_state_flag(GameStateFlags::TitleCards);
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
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &utility_tab.tab, OPTIONS_IDX)
    }
}

impl PreferencesItems {
    fn execute(&self) {
        match self {
            Self::FpsCap => {
                send_input_event!(text, _app, {
                    if let Ok(v) = text.parse() {
                        Preferences::update_er(|c| {
                            c.fps = Some(v);
                        })
                        .send_error();
                    } else if text.is_empty() {
                        Preferences::update_er(|c| {
                            c.fps = None;
                        })
                        .send_error();
                    }
                })
            }
            Self::NoDeath => {
                Preferences::update_er(|c| c.no_death = !c.no_death).send_error()
            }
            Self::NoDamage => {
                Preferences::update_er(|c| c.no_damage = !c.no_damage).send_error()
            }
            Self::SetRfbsOnLoad => {
                Preferences::update_er(|c| c.rfbs_on_load = !c.rfbs_on_load).send_error()
            }
            Self::InfinitePoise => {
                Preferences::update_er(|c| c.infinite_poise = !c.infinite_poise).send_error()
            }
            Self::MuteMusic => {
                Preferences::update_er(|c| c.mute_music = !c.mute_music).send_error()
            }
            Self::RemoveLogos => {
                Preferences::update_er(|c| c.remove_logo = !c.remove_logo).send_error()
            }
            Self::StutterFix => {
                Preferences::update_er(|c| c.stutter_fix = !c.stutter_fix).send_error()
            }
            Self::DisableAreaTitleCards => {
                Preferences::update_er(|c| c.disable_area_target_cards = !c.disable_area_target_cards).send_error()
            }
        }
    }
    fn set_input(&self) {
        match self {
            Self::FpsCap => self.execute(),
            _ => (),
        }
    }
    fn to_list_item(&self, preferences: &ErPreferences) -> ListItem<'_> {
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
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(&utility_tab.preferences.elden_ring)).collect();
        blockless_list(items, &utility_tab.tab, PREFERENCES_IDX)
    }
}
