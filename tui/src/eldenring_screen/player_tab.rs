use crate::{
    common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list},
    eldenring_screen::GameState,
    event::{AnyhowExt, InfoType, ResultExt, send_event, Event},
    input::request_input,
    spawn_task,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use eldenring::{
    chr_ins::ChrInsExt,
    emevd,
    game_state::{StateFlagOffset, StateFlags},
    player::{
        self, ChrDbgOffset, PlayerGameData, PlayerGameDataOffset, Position, format_position,
        is_chr_dbg_flag, save_position, torrent_ins,
    },
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem},
};

enum ActionsItems {
    SetHealth,
    Die,
    GiveRunes,
    AnimationSpeed,
    Rest,
    SavePos1,
    SavePos2,
    RestorePos1,
    RestorePos2,
}

enum TogglesItems {
    NoDeath,
    NoDamage,
    InfinitePoise,
    SetRfbsOnLoad,
    OneShot,
    RuneArc,
    Silent,
    Hidden,
    InfiniteStamina,
    InfiniteFp,
    InfiniteConsumables,
    InfiniteArrows,
    TorrentAnywhere,
    TorrentNoDeath,
    Set1hp,
    NoTimeChangeDeath,
    NoRuneLoss,
    DisableAchievements,
}

pub enum Stats {
    Vigor,
    Mind,
    Endurance,
    Strength,
    Dexterity,
    Intelligence,
    Faith,
    Arcane,
    Scadutree,
    SpiritAsh,
}

const TOGGLES_IDX: usize = 0;
const ACTIONS_IDX: usize = 1;
pub const STATS_IDX: usize = 2;

pub struct PlayerTab {
    tab: TabState,
    stats: PlayerGameData,
}

impl PlayerTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[STATS_IDX] = StatefulList::new(0);
        PlayerTab {
            tab: TabState::new(list_states),
            stats: PlayerGameData::read(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect) {
        self.stats = PlayerGameData::read();

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
        if self.tab.current_list == STATS_IDX {
            self.tab.set_length(STATS_IDX, Stats::array().len());
        }

        self.tab.handle_keys(key);

        match key.code {
            KeyCode::Enter => self.handle_enter(),
            _ => (),
        }

        if self.tab.current_list == STATS_IDX &&
        let Some(selected_idx) = self.tab.lists_states[STATS_IDX].selected() {
            match key.code {
                KeyCode::Char('h') => {
                    Stats::array()[selected_idx]
                        .increment_stat(&self.stats, -1)
                        .send_error();
                }
                KeyCode::Char('l') => {
                    Stats::array()[selected_idx]
                        .increment_stat(&self.stats, 1)
                        .send_error();
                }
                _ => (),
            }
        }
    }

    fn handle_enter(&mut self) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].execute(),
                TOGGLES_IDX => TogglesItems::ARRAY[selected_index].execute(&self.stats),
                STATS_IDX => {
                    spawn_task! {
                        if let Some(val) = request_input::<i32>(None).await {
                            let stat  = &Stats::array()[selected_index];
                            stat.set_stat(val).send_error();
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn get_saved_pos_display(&self, index: usize) -> String {
        match Position::read_from_cave(index) {
            Ok(pos) if pos.is_valid() => format_position(&pos),
            _ => "Not set".to_string(),
        }
    }
}

impl ActionsItems {
    fn execute(&self) {
        match self {
            Self::SetHealth => {
                spawn_task! {
                    if let Some(val) = request_input::<i32>(None).await {
                        player::player_ins().set_hp(val).send_error();
                    }
                }
            }
            Self::GiveRunes => {
                spawn_task! {
                    if let Some(val) = request_input::<u32>(None).await {
                        player::set_runes(val).send_error();
                    }
                }
            }
            Self::AnimationSpeed => {
                spawn_task! {
                    if let Some(val) = request_input::<f32>(None).await {
                        GameState::player_ins().set_animation_speed(val).send_error()
                    }
                }
            }
            Self::Die => GameState::player_ins().set_hp(0).send_error(),
            Self::Rest => emevd::rest().send_error(),
            Self::SavePos1 => {
                spawn_task! {
                    save_position(0).send_error()
                }
            }
            Self::SavePos2 => {
                spawn_task! {
                    save_position(1).send_error()
                }
            }
            Self::RestorePos1 => {
                spawn_task! {
                    match player::restore_position(0).await {
                        Ok(()) => {},
                        Err(e) => {
                            // Only show error if the position is actually saved but restore failed
                            // (not just "not set")
                            match Position::read_from_cave(0) {
                                Ok(pos) if pos.is_valid() => {
                                    eprintln!("Failed to restore position 1: {}", e);
                                }
                                _ => {
                                    // Position not set - this is expected, no error needed
                                }
                            }
                        }
                    }
                }
            }
            Self::RestorePos2 => {
                spawn_task! {
                    match player::restore_position(1).await {
                        Ok(()) => {},
                        Err(e) => {
                            match Position::read_from_cave(1) {
                                Ok(pos) if pos.is_valid() => {
                                    eprintln!("Failed to restore position 2: {}", e);
                                }
                                _ => {
                                    // Position not set - this is expected, no error needed
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn to_list_item(&self, player_tab: &PlayerTab) -> ListItem<'static> {
        let text = match self {
            Self::SetHealth => {
                format!("Health: {}", player_tab.stats.current_hp)
            }
            Self::Die => {
                "Die".to_string()
            }
            Self::Rest => {
                "Rest".to_string()
            }
            Self::GiveRunes => {
                format!("Runes: {}", player_tab.stats.rune_count)
            }
            Self::AnimationSpeed => {
                format!("Animation Speed: {}",
                    GameState::player_ins().get_animation_speed().unwrap_or_default())
            }
            Self::SavePos1 => {
                format!("Save Pos 1: {}", player_tab.get_saved_pos_display(0))
            }
            Self::SavePos2 => {
                format!("Save Pos 2: {}", player_tab.get_saved_pos_display(1))
            }
            Self::RestorePos1 => {
                "Restore Pos 1".to_string()
            }
            Self::RestorePos2 => {
                "Restore Pos 2".to_string()
            }
        };
        ListItem::new(text)
    }
    const ARRAY: &[ActionsItems] = &[
        Self::SetHealth,
        Self::GiveRunes,
        Self::AnimationSpeed,
        Self::Die,
        Self::Rest,
        Self::SavePos1,
        Self::SavePos2,
        Self::RestorePos1,
        Self::RestorePos2,
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(player_tab)).collect();
        tabs_list(items, None, &player_tab.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self, stats: &PlayerGameData) {
        match self {
            Self::NoDeath => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffset::PlayerNoDeath).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffset::PlayerNoDeath, new_state).send_error();
            }
            Self::NoDamage => {
                let new_state = !GameState::state_flags().player_no_damage;
                StateFlags::set(StateFlagOffset::PlayerNoDamage, new_state).send_error();
                GameState::player_ins().set_no_damage(new_state).ok();
            }
            Self::SetRfbsOnLoad => {
                let new_state = !GameState::state_flags().rfbs;
                StateFlags::set(StateFlagOffset::Rfbs, new_state).send_error();
            }
            Self::InfinitePoise => {
                let new_state = !player::is_infinite_poise().unwrap_or_default();
                player::set_infinite_poise(new_state).send_error();
            }
            Self::OneShot => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffset::OneShot).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffset::OneShot, new_state).send_error();
            }
            Self::RuneArc => {
                let new_state = !(stats.rune_arc_active || GameState::state_flags().rune_arc);
                StateFlags::set(StateFlagOffset::RuneArc, new_state).send_error();
                player::set_rune_arc(new_state).ok();
            }
            Self::InfiniteStamina => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffset::InfiniteStam).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffset::InfiniteStam , new_state).send_error();
            }
            Self::InfiniteFp => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffset::InfiniteFp).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffset::InfiniteFp, new_state).send_error();
            }
            Self::InfiniteConsumables => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffset::InfiniteGoods).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffset::InfiniteGoods, new_state).send_error();
            }
            Self::Hidden => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffset::Hidden).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffset::Hidden, new_state).send_error();
            }
            Self::Silent => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffset::Silent).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffset::Silent, new_state).send_error();
            }
            Self::InfiniteArrows => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffset::InfiniteArrows).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffset::InfiniteArrows, new_state).send_error();
            }
            Self::TorrentNoDeath => {
                let new_state = !GameState::state_flags().torrent_no_death;
                StateFlags::set(StateFlagOffset::TorrentNoDeath, new_state).send_error();
                let torrent_ins = torrent_ins();
                torrent_ins.set_no_death(!torrent_ins.is_no_death().unwrap_or_default()).ok();
            }
            Self::TorrentAnywhere => {
                let new_state = !player::is_torrent_anywhere().unwrap_or_default();
                player::set_torrent_anywhere(new_state).send_error();
            }
            Self::Set1hp => {
                let new_state = !GameState::state_flags().set_1hp;
                StateFlags::set(StateFlagOffset::Set1hp, new_state).send_error();
                // Apply immediately if toggling on
                if new_state {
                    spawn_task! {
                        player::set_1hp().send_error();
                    }
                } else {
                    // Warn user that HP is not restored
                    send_event(Event::Info((
                        "Set 1 HP disabled. Player remains at 1 HP. Use 'Set HP' to restore.".to_string(),
                        InfoType::GameError,
                    )));
                }
            }
            Self::NoTimeChangeDeath => {
                let new_state = !GameState::state_flags().no_time_change_death;
                StateFlags::set(StateFlagOffset::NoTimeChangeDeath, new_state).send_error();
                // Apply immediately if toggling on
                if new_state {
                    emevd::init_time_of_day().send_error();
                }
            }
            Self::DisableAchievements => {
                let new_state = !GameState::state_flags().disable_achievements;
                StateFlags::set(StateFlagOffset::DisableAchievements, new_state).send_error();
                // Apply immediately if toggling on
                if new_state {
                    eldenring::achievement::install_patch().send_error();
                } else {
                    eldenring::achievement::uninstall_patch().send_error();
                }
            }
            Self::NoRuneLoss => {
                let new_state = !GameState::state_flags().no_rune_loss;
                StateFlags::set(StateFlagOffset::NoRuneLoss, new_state).send_error();
                // Apply immediately if toggling on
                if new_state {
                    eldenring::no_rune_loss::install_patch().send_error();
                } else {
                    eldenring::no_rune_loss::uninstall_patch().send_error();
                }
            }
        }
    }
    fn to_list_item(&self, player_tab: &PlayerTab) -> ListItem<'_> {
        let text = match self {
            Self::NoDeath => {
                let state = player::is_chr_dbg_flag(ChrDbgOffset::PlayerNoDeath).unwrap_or_default();
                "No Death".create_toggle_str(state)
            }
            Self::NoDamage => {
                let state = GameState::state_flags().player_no_damage;
                "No Damage".create_toggle_str(state)
            }
            Self::SetRfbsOnLoad => {
                let state = GameState::state_flags().rfbs;
                "Set RFBS on load".create_toggle_str(state)
            }
            Self::InfinitePoise => {
                let state = player::is_infinite_poise().unwrap_or_default();
                "Infinite Poise".create_toggle_str(state)
            }
            Self::OneShot => {
                let state = player::is_chr_dbg_flag(ChrDbgOffset::OneShot).unwrap_or_default();
                "One Shot".create_toggle_str(state)
            }
            Self::RuneArc => {
                let state = player_tab.stats.rune_arc_active || GameState::state_flags().rune_arc;
                "Rune Arc".create_toggle_str(state)
            }
            Self::InfiniteStamina => {
                let state = player::is_chr_dbg_flag(ChrDbgOffset::InfiniteStam).unwrap_or_default();
                "Infinite Stamina".create_toggle_str(state)
            }
            Self::InfiniteFp => {
                let state = player::is_chr_dbg_flag(ChrDbgOffset::InfiniteFp).unwrap_or_default();
                "Infinite FP".create_toggle_str(state)
            }
            Self::InfiniteConsumables => {
                let state = player::is_chr_dbg_flag(ChrDbgOffset::InfiniteGoods).unwrap_or_default();
                "Infinite Consumables".create_toggle_str(state)
            }
            Self::Silent => {
                let state = player::is_chr_dbg_flag(ChrDbgOffset::Silent).unwrap_or_default();
                "Silent".create_toggle_str(state)
            }
            Self::Hidden => {
                let state = player::is_chr_dbg_flag(ChrDbgOffset::Hidden).unwrap_or_default();
                "Hidden".create_toggle_str(state)
            }
            Self::InfiniteArrows => {
                let state = player::is_chr_dbg_flag(ChrDbgOffset::InfiniteArrows).unwrap_or_default();
                "Infinite Arrows".create_toggle_str(state)
            }
            Self::TorrentNoDeath => {
                let state = GameState::state_flags().torrent_no_death;
                "Torrent No Death".create_toggle_str(state)
            }
            Self::TorrentAnywhere=> {
                let state = player::is_torrent_anywhere().unwrap_or_default();
                "Torrent Anywhere".create_toggle_str(state)
            }
            Self::Set1hp => {
                let state = GameState::state_flags().set_1hp;
                "Set 1 HP".create_toggle_str(state)
            }
            Self::NoTimeChangeDeath => {
                let state = GameState::state_flags().no_time_change_death;
                "No Time Change on Death".create_toggle_str(state)
            }
            Self::DisableAchievements => {
                let state = GameState::state_flags().disable_achievements;
                "Disable Achievements".create_toggle_str(state)
            }
            Self::NoRuneLoss => {
                let state = GameState::state_flags().no_rune_loss;
                "No Rune Loss on Death".create_toggle_str(state)
            }
        };
        ListItem::from(text)
    }
    const ARRAY: &[TogglesItems] = &[
        Self::NoDeath,
        Self::NoDamage,
        Self::InfinitePoise,
        Self::OneShot,
        Self::RuneArc,
        Self::SetRfbsOnLoad,
        Self::Silent,
        Self::Hidden,
        Self::InfiniteStamina,
        Self::InfiniteFp,
        Self::InfiniteConsumables,
        Self::InfiniteArrows,
        Self::TorrentAnywhere,
        Self::TorrentNoDeath,
        Self::Set1hp,
        Self::NoTimeChangeDeath,
        Self::NoRuneLoss,
        Self::DisableAchievements,
    ];
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(player_tab)).collect();
        tabs_list(items, None, &player_tab.tab, TOGGLES_IDX)
    }
}

impl Stats {
    fn to_list_item(&self, stats: &PlayerGameData) -> ListItem<'_> {
        let text = match self {
            Self::Vigor => format!("{:02} Vigor", stats.vigor),
            Self::Mind => format!("{:02} Mind", stats.mind),
            Self::Endurance => format!("{:02} Endurance", stats.endurance),
            Self::Strength => format!("{:02} Strength", stats.strength),
            Self::Dexterity => format!("{:02} Dexterity", stats.dexterity),
            Self::Intelligence => format!("{:02} Intelligence", stats.intelligence),
            Self::Faith => format!("{:02} Faith", stats.faith),
            Self::Arcane => format!("{:02} Arcane", stats.arcane),
            Self::Scadutree => format!("{:02} Scadutree", stats.scadutree_blessing),
            Self::SpiritAsh => format!("{:02} Spirit Ash", stats.reversed_spirit_ash),
        };
        ListItem::from(text)
    }

    pub fn set_stat(&self, val: i32) -> Result<()> {
        match self {
            Self::Vigor => player::set_stat(PlayerGameDataOffset::Vigor, val),
            Self::Mind => player::set_stat(PlayerGameDataOffset::Mind, val),
            Self::Endurance => player::set_stat(PlayerGameDataOffset::Endurance, val),
            Self::Strength => player::set_stat(PlayerGameDataOffset::Strength, val),
            Self::Dexterity => player::set_stat(PlayerGameDataOffset::Dexterity, val),
            Self::Intelligence => player::set_stat(PlayerGameDataOffset::Intelligence, val),
            Self::Faith => player::set_stat(PlayerGameDataOffset::Faith, val),
            Self::Arcane => player::set_stat(PlayerGameDataOffset::Arcane, val),
            Self::Scadutree => player::set_dlc_stat(PlayerGameDataOffset::Scadutree, val as u8),
            Self::SpiritAsh => player::set_dlc_stat(PlayerGameDataOffset::SpiritAsh, val as u8),
        }
    }
    fn increment_stat(&self, stats: &PlayerGameData, val: i32) -> Result<()> {
        match self {
            Self::Vigor => self.set_stat(stats.vigor as i32+ val),
            Self::Mind => self.set_stat(stats.mind as i32 + val),
            Self::Endurance => self.set_stat(stats.endurance as i32 + val),
            Self::Strength => self.set_stat(stats.strength as i32 + val),
            Self::Dexterity => self.set_stat(stats.dexterity as i32 + val),
            Self::Intelligence => self.set_stat(stats.intelligence as i32 + val),
            Self::Faith => self.set_stat(stats.faith as i32 + val),
            Self::Arcane => self.set_stat(stats.arcane as i32 + val),
            Self::Scadutree => self.set_stat(stats.scadutree_blessing as i32 + val),
            Self::SpiritAsh => self.set_stat(stats.reversed_spirit_ash as i32 + val),
        }
    }
    const ARRAY: &[Stats] = &[
        Self::Vigor,
        Self::Mind,
        Self::Endurance,
        Self::Strength,
        Self::Dexterity,
        Self::Intelligence,
        Self::Faith,
        Self::Arcane,
        Self::Scadutree,
        Self::SpiritAsh,
    ];
    const NO_DLC_ARRAY: &[Stats] = &[
        Self::Vigor,
        Self::Mind,
        Self::Endurance,
        Self::Strength,
        Self::Dexterity,
        Self::Intelligence,
        Self::Faith,
        Self::Arcane,
    ];
    pub fn array() -> &'static [Stats] {
        if GameState::dlc() { Self::ARRAY } else { Self::NO_DLC_ARRAY }
    }
    fn list(player_tab: &PlayerTab) -> List<'static> {
        let array = Self::array();
        let items: Vec<ListItem> = array.iter().map(|i| i.to_list_item(&player_tab.stats)).collect();
        tabs_list(items, Some("Stats"), &player_tab.tab, STATS_IDX)
    }
}