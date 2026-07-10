use crate::{
    common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list},
    eldenring_screen::GameState,
    event::ResultExt,
    input::request_input,
    spawn_task,
    theme::theme,
};
use config::{Config, attach::AttachConfig};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use eldenring::{
    emevd,
    game_state::{StateFlagOffset, StateFlags},
    resources::talk_commands::{MENUS, shops_array},
    utility::{self, ControlFlag},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    symbols,
    widgets::{List, ListItem, Tabs},
};

enum TogglesItems {
    RemoveLogos,
    ToggleMusic,
    ShowAllMaps,
    ShowAllGraces,
    StutterFix,
    FreezeWorld,
    DisableAreaTitleCards,
    DrawHitboxesA,
    MapAnywhere,
    TravelAnywhere,
    DisableRoll,
    DisableJump,
    DisableBackstep,
}

enum ActionsItems {
    FpsCap,
    GameSpeed,
    SetTimeOfDay,
    Quitout,
    ClearCount,
    TriggerNewGameCycle,
}

const OPTIONS_IDX: usize = 0;
const ACTIONS_IDX: usize = 1;
const MENUS_IDX: usize = 2;
const SHOPS_IDX: usize = 3;

pub struct UtilityTab {
    tab: TabState,
    preferences: AttachConfig,
    menu_shop_idx: usize,
}

impl UtilityTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 4];
        list_states[OPTIONS_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[MENUS_IDX] = StatefulList::new(MENUS.len());
        list_states[SHOPS_IDX] = StatefulList::new(0);
        UtilityTab {
            tab: TabState::new(list_states),
            preferences: AttachConfig::read().unwrap_or_default(),
            menu_shop_idx: 0,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        self.preferences = AttachConfig::read().unwrap_or_default();

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

        frame.render_stateful_widget(
            TogglesItems::list(self),
            layout[OPTIONS_IDX],
            &mut self.tab.get_list_state(OPTIONS_IDX),
        );
        frame.render_stateful_widget(
            ActionsItems::list(self),
            layout[ACTIONS_IDX],
            &mut self.tab.get_list_state(ACTIONS_IDX),
        );

        if self.menu_shop_idx == 1 {
            frame.render_stateful_widget(
                self.shops_list(),
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

    pub fn handle_keys(&mut self, key: KeyEvent) {
        if self.tab.current_list == SHOPS_IDX {
            self.tab.set_length(SHOPS_IDX, shops_array(GameState::dlc()).len())
        }

        self.tab.handle_keys(key);

        match self.tab.current_list {
            MENUS_IDX => {
                if self.menu_shop_idx == 1 {
                    self.tab.current_list = SHOPS_IDX
                }
                if key.code == KeyCode::Char('l') {
                    self.tab.current_list = SHOPS_IDX;
                    self.menu_shop_idx = 1
                }
            }
            SHOPS_IDX => match (key.code, key.modifiers) {
                (KeyCode::Char('h'), _) => {
                    self.tab.current_list = MENUS_IDX;
                    self.menu_shop_idx = 0
                }
                (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                    self.tab.current_list = 1;
                }
                _ => (),
            },
            _ => (),
        }

        if key.code == KeyCode::Enter {
            self.handle_enter();
        }
    }

    fn handle_enter(&mut self) {
        if let Some(selected) = self.tab.get_list_selected(self.tab.current_list) {
            match self.tab.current_list {
                OPTIONS_IDX => TogglesItems::ARRAY[selected].execute(),
                ACTIONS_IDX => ActionsItems::ARRAY[selected].execute(),
                MENUS_IDX => MENUS[selected].execute().send_error(),
                SHOPS_IDX => shops_array(GameState::dlc())[selected].execute().send_error(),
                _ => (),
            }
        }
    }

    fn menus_list(&self) -> List<'static> {
        let items: Vec<ListItem> = MENUS.iter().map(|menu| ListItem::new(menu.name)).collect();
        tabs_list(items, None, &self.tab, MENUS_IDX)
    }

    fn shops_list(&self) -> List<'static> {
        let items: Vec<ListItem> = shops_array(GameState::dlc()).iter().map(|shop| ListItem::from(shop.name)).collect();
        tabs_list(items, None, &self.tab, SHOPS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self) {
        match self {
            Self::ToggleMusic => {
                let new_state = !utility::is_music_muted();
                utility::mute_music(new_state).send_error()
            }
            Self::RemoveLogos => {
                let new_state = !utility::is_logo_patch();
                utility::set_logo_patch(new_state).send_error()
            }
            Self::ShowAllMaps => {
                let new_state = !utility::is_show_all_maps_on();
                utility::show_all_maps(new_state).send_error()
            }
            Self::ShowAllGraces => {
                let new_state = !utility::is_show_all_graces_on();
                utility::show_all_graces(new_state).send_error()
            }
            Self::StutterFix => {
                let new_state = !GameState::state_flags().stutter_fix;
                StateFlags::set(StateFlagOffset::StutterFix, new_state).send_error();
                let _ = utility::set_stutter_fix(new_state);
            }
            Self::FreezeWorld => {
                let new_state = !utility::is_freeze_world_on();
                utility::set_freeze_world(new_state).send_error()
            }
            Self::DisableAreaTitleCards => {
                let new_state = !GameState::state_flags().title_cards;
                StateFlags::set(StateFlagOffset::TitleCards, new_state).send_error();
            }
            Self::DrawHitboxesA => {
                let new_state = !GameState::state_flags().hitboxes;
                StateFlags::set(StateFlagOffset::Hitboxes, new_state).send_error();
                let _ = utility::draw_hitboxes(new_state, false);
            }
            Self::MapAnywhere => {
                let new_state = !utility::is_map_anywhere();
                utility::set_map_anywhere(new_state).send_error()
            }
            Self::TravelAnywhere => {
                let new_state = !utility::is_travel_anywhere();
                utility::set_travel_anywhere(new_state).send_error()
            }
            Self::DisableRoll => {
                let new_state = !utility::is_control_disabled(ControlFlag::Roll);
                utility::set_control(ControlFlag::Roll, new_state).send_error()
            }
            Self::DisableJump => {
                let new_state = !utility::is_control_disabled(ControlFlag::Jump);
                utility::set_control(ControlFlag::Jump, new_state).send_error()
            }
            Self::DisableBackstep => {
                let new_state = !utility::is_control_disabled(ControlFlag::Backstep);
                utility::set_control(ControlFlag::Backstep, new_state).send_error()
            }
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::ToggleMusic => {
                let state = utility::is_music_muted();
                "Mute Music".create_toggle_str(state)
            }
            Self::RemoveLogos => {
                let state = utility::is_logo_patch();
                "Remove Logos".create_toggle_str(state)
            }
            Self::ShowAllMaps => {
                let state = utility::is_show_all_maps_on();
                "Show All Maps".create_toggle_str(state)
            }
            Self::ShowAllGraces => {
                let state = utility::is_show_all_graces_on();
                "Show All Graces".create_toggle_str(state)
            }
            Self::StutterFix => {
                let state = GameState::state_flags().stutter_fix;
                "Stutter Fix".create_toggle_str(state)
            }
            Self::FreezeWorld => {
                let state = utility::is_freeze_world_on();
                "Freeze World".create_toggle_str(state)
            }
            Self::DisableAreaTitleCards => {
                let state = GameState::state_flags().title_cards;
                "Disable Area Title Cards".create_toggle_str(state)
            }
            Self::DrawHitboxesA => {
                let state = GameState::state_flags().hitboxes;
                "Draw Hitboxes".create_toggle_str(state)
            }
            Self::MapAnywhere => {
                let state = utility::is_map_anywhere();
                "Allow Map In Combat".create_toggle_str(state)
            }
            Self::TravelAnywhere => {
                let state = utility::is_travel_anywhere();
                "Allow Travel In Dungeons".create_toggle_str(state)
            }
            Self::DisableRoll => {
                let state = utility::is_control_disabled(ControlFlag::Roll);
                "Disable Roll".create_toggle_str(state)
            }
            Self::DisableJump => {
                let state = utility::is_control_disabled(ControlFlag::Jump);
                "Disable Jump".create_toggle_str(state)
            }
            Self::DisableBackstep => {
                let state = utility::is_control_disabled(ControlFlag::Backstep);
                "Disable Backstep".create_toggle_str(state)
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[TogglesItems] = &[
        Self::FreezeWorld,
        Self::ToggleMusic,
        Self::RemoveLogos,
        Self::DisableAreaTitleCards,
        Self::DrawHitboxesA,
        Self::MapAnywhere,
        Self::TravelAnywhere,
        Self::ShowAllGraces,
        Self::ShowAllMaps,
        Self::StutterFix,
        Self::DisableRoll,
        Self::DisableJump,
        Self::DisableBackstep,
    ];
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &utility_tab.tab, OPTIONS_IDX)
    }
}

impl ActionsItems {
    fn execute(&self) {
        match self {
            Self::FpsCap => {
                spawn_task! {
                    if let Some(val) = request_input::<f32>(None).await {
                        utility::set_fps_cap(val).send_error()
                    }
                }
            }
            Self::GameSpeed => {
                spawn_task! {
                    if let Some(val) = request_input::<f32>(None).await {
                        utility::set_game_speed(val).send_error()
                    }
                }
            }
            Self::SetTimeOfDay => {
                spawn_task! {
                    if let Some(val) = request_input::<f32>(Some("Enter time (0-23.999): ")).await {
                        emevd::set_time_of_day(val).send_error()
                    }
                }
            }
            Self::ClearCount => {
                spawn_task! {
                    if let Some(val) = request_input::<i32>(None).await {
                        utility::set_ng_cycle(val).send_error()
                    }
                }
            }
            Self::Quitout => utility::quitout().send_error(),
            Self::TriggerNewGameCycle => utility::trigger_new_game().send_error(),
        }
    }
    fn to_list_item(&self) -> ListItem<'_> {
        let text = match self {
            Self::FpsCap => {
                format!("FPS Cap: {}",
                    utility::get_fps_cap())
            }
            Self::GameSpeed => {
                format!("Game Speed: {}", utility::get_game_speed())
            }
            Self::SetTimeOfDay => {
                let time = emevd::get_time_of_day();
                let hour = (time as i32) % 24;
                let minute = ((time - hour as f32) * 60.0) as i32;
                format!("Set Time of Day: {:02}:{:02}", hour, minute)
            }
            Self::Quitout => {
                "Quitout".to_string()
            }
            Self::ClearCount => {
                format!("ClearCount: {}",
                    utility::get_ng_cycle().unwrap_or_default())
            }
            Self::TriggerNewGameCycle => {
                "Trigger New Game Cycle".to_string()
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
        Self::FpsCap,
        Self::GameSpeed,
        Self::SetTimeOfDay,
        Self::ClearCount,
        Self::TriggerNewGameCycle,
        Self::Quitout,
    ];
    fn list(utility_tab: &UtilityTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item()).collect();
        tabs_list(items, None, &utility_tab.tab, ACTIONS_IDX)
    }
}