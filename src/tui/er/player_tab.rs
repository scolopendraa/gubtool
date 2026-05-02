use crate::{
    config::ui_state::UiState,
    er::{
        chr_ins::{ChrIns, ChrInsExt},
        event,
        game_state::{self, GameStateFlags, get_state_flag},
        offsets::{chr_dbg_flags::ChrDbgOffsets, game_data_man::player_game_data_offsets},
        player::{self, PlayerStats, is_chr_dbg_flag, torrent_ins},
    },
    send_input_event,
    tui::{
        common::{StrExt, stateful_list::StatefulList, tab_state::TabState, tabs_list},
        er::ErInfo,
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
    SetHealth,
    Die,
    GiveRunes,
    AnimationSpeed,
    Rest,
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
    RuneLevel,
    RuneMem,
}

const TOGGLES_IDX: usize = 0;
const ACTIONS_IDX: usize = 1;
pub const STATS_IDX: usize = 2;

pub struct PlayerTab {
    pub tab: TabState,
    pub stats: PlayerStats,
    pub hp: i32,
    pub runes: i64,
}

impl PlayerTab {
    pub fn new() -> Self {
        let mut list_states = vec![StatefulList::new(0); 3];
        list_states[TOGGLES_IDX] = StatefulList::new(TogglesItems::ARRAY.len());
        list_states[ACTIONS_IDX] = StatefulList::new(ActionsItems::ARRAY.len());
        list_states[STATS_IDX] = StatefulList::new(0);
        PlayerTab {
            tab: TabState::new(list_states),
            stats: PlayerStats::new(),
            hp: 100,
            runes: 10000,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, layout: Rect, er: &ErInfo) {
        self.stats.update().ok();

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
            ActionsItems::list(self, &er.player_ins),
            layout[ACTIONS_IDX],
            &mut self.tab.get_list_state(ACTIONS_IDX),
        );
        frame.render_stateful_widget(
            TogglesItems::list(self, &er.player_ins),
            layout[TOGGLES_IDX],
            &mut self.tab.get_list_state(TOGGLES_IDX),
        );
        frame.render_stateful_widget(
            Stats::list(self, er.dlc),
            layout[STATS_IDX],
            &mut self.tab.get_list_state(STATS_IDX),
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent, er: &ErInfo) {
        if self.tab.current_list == STATS_IDX {
            self.tab.set_length(STATS_IDX, Stats::array(er.dlc).len());
        }

        self.tab.handle_keys(key);
        match key.code {
            KeyCode::Char('s') => self.handle_input(),
            KeyCode::Enter => self.handle_enter(&er.player_ins),
            _ => (),
        }
        if self.tab.current_list == STATS_IDX &&
        let Some(selected_idx) = self.tab.lists_states[STATS_IDX].selected() {
            match key.code {
                KeyCode::Char('h') => {
                    Stats::array(er.dlc)[selected_idx]
                        .increment_stat(&self.stats, -1)
                        .send_error();
                }
                KeyCode::Char('l') => {
                    Stats::array(er.dlc)[selected_idx]
                        .increment_stat(&self.stats, 1)
                        .send_error();
                }
                _ => (),
            }
        }
    }
    fn handle_input(&mut self) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].set_input(),
                STATS_IDX => Stats::ARRAY[selected_index].set_input(),
                _ => (),
            }
        }
    }
    fn handle_enter(&mut self, player_ins: &ChrIns) {
        let current_list = self.tab.current_list;
        if let Some(selected_index) = self.tab.lists_states[current_list].selected() {
            match current_list {
                ACTIONS_IDX => ActionsItems::ARRAY[selected_index].execute(self, player_ins),
                TOGGLES_IDX => TogglesItems::ARRAY[selected_index].execute(&self.stats, player_ins),
                STATS_IDX => Stats::ARRAY[selected_index].set_input(),
                _ => (),
            }
        }
    }
}

impl ActionsItems {
    fn execute(&self, player_tab: &PlayerTab, player_ins: &ChrIns) {
        match self {
            Self::SetHealth => player_ins.set_hp(player_tab.hp).send_error(),
            Self::Die => player_ins.set_hp(0).send_error(),
            Self::Rest => event::rest().send_error(),
            Self::GiveRunes => player::give_runes(player_tab.runes).send_error(),
            Self::AnimationSpeed => {},
        }
    }
    fn set_input(&self) {
        match self {
            Self::SetHealth => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.elden_ring.player.hp = v;
                        UiState::update_er(|c| { c.player_set_health = v; }).ok();
                    }
                });
            },
            Self::GiveRunes => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.elden_ring.player.runes = v;
                        UiState::update_er(|c| { c.give_runes = v; }).ok();
                    }
                });
            },
            Self::AnimationSpeed => {
                send_input_event!(text, app, {
                    if let Ok(v) = text.parse() {
                        app.elden_ring.er_info.player_ins.set_animation_speed(v)
                        .send_error()
                    }
                });
            },
            _ => (),
        }
    }
    fn to_list_item(&self, player_tab: &PlayerTab, player_ins: &ChrIns) -> ListItem<'static> {
        let text = match self {
            Self::SetHealth => {
                format!("Set Health ({})", player_tab.hp)
            }
            Self::Die => {
                "Die".to_string()
            }
            Self::Rest => {
                "Rest".to_string()
            }
            Self::GiveRunes => {
                format!("Give Runes ({})", player_tab.runes)
            }
            Self::AnimationSpeed => {
                format!("Animation Speed: {}",
                    player_ins.get_animation_speed().unwrap_or_default())
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
    ];
    fn list(player_tab: &PlayerTab, player_ins: &ChrIns) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(player_tab, player_ins)).collect();
        tabs_list(items, None, &player_tab.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self, stats: &PlayerStats, player_ins: &ChrIns) {
        match self {
            Self::NoDeath => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, new_state).send_error();
            }
            Self::NoDamage => {
                let new_state = !(player_ins.is_no_damage().unwrap_or_default() ||
                    get_state_flag(GameStateFlags::PlayerNoDamage));
                game_state::set_state_flag(GameStateFlags::PlayerNoDamage, new_state).send_error();
                player_ins.set_no_damage(new_state).ok();
            }
            Self::SetRfbsOnLoad => {
                let new_state = !game_state::get_state_flag(GameStateFlags::Rfbs);
                game_state::set_state_flag(GameStateFlags::Rfbs, new_state).send_error();
            }
            Self::InfinitePoise => {
                let new_state = !player::is_infinite_poise().unwrap_or_default();
                player::set_infinite_poise(new_state).send_error();
            }
            Self::OneShot => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::OneShot).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::OneShot, new_state).send_error();
            }
            Self::RuneArc => {
                let new_state = !(stats.rune_arc ||
                    get_state_flag(GameStateFlags::RuneArc));
                game_state::set_state_flag(GameStateFlags::RuneArc, new_state).send_error();
                player::set_rune_arc(new_state).ok();
            }
            Self::InfiniteStamina => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteStam).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteStam , new_state).send_error();
            }
            Self::InfiniteFp => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteFp).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteFp, new_state).send_error();
            }
            Self::InfiniteConsumables => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteGoods).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteGoods, new_state).send_error();
            }
            Self::Hidden => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::Hidden).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::Hidden, new_state).send_error();
            }
            Self::Silent => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::Silent).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::Silent, new_state).send_error();
            }
            Self::InfiniteArrows => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteArrows).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteArrows, new_state).send_error();
            }
            Self::TorrentNoDeath => {
                let new_state = !game_state::get_state_flag(GameStateFlags::TorrentNoDeath);
                game_state::set_state_flag(GameStateFlags::TorrentNoDeath, new_state).send_error();
                let torrent_ins = torrent_ins();
                torrent_ins.set_no_death(!torrent_ins.is_no_death().unwrap_or_default()).ok();
            }
            Self::TorrentAnywhere => {
                let new_state = !player::is_torrent_anywhere().unwrap_or_default();
                player::set_torrent_anywhere(new_state).send_error();
            }
        }
    }
    fn to_list_item(&self, player_tab: &PlayerTab, player_ins: &ChrIns) -> ListItem<'_> {
        let text = match self {
            Self::NoDeath => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath).unwrap_or_default();
                "No Death".create_toggle_str(state)
            }
            Self::NoDamage => {
                let state = player_ins.is_no_damage().unwrap_or_default() ||
                    get_state_flag(GameStateFlags::PlayerNoDamage);
                "No Damage".create_toggle_str(state)
            }
            Self::SetRfbsOnLoad => {
                let state = get_state_flag(GameStateFlags::Rfbs);
                "Set RFBS on load".create_toggle_str(state)
            }
            Self::InfinitePoise => {
                let state = player::is_infinite_poise().unwrap_or_default();
                "Infinite Poise".create_toggle_str(state)
            }
            Self::OneShot => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::OneShot).unwrap_or_default();
                "One Shot".create_toggle_str(state)
            }
            Self::RuneArc => {
                let state = player_tab.stats.rune_arc ||
                    get_state_flag(GameStateFlags::RuneArc);
                "Rune Arc".create_toggle_str(state)
            }
            Self::InfiniteStamina => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::InfiniteStam).unwrap_or_default();
                "Infinite Stamina".create_toggle_str(state)
            }
            Self::InfiniteFp => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::InfiniteFp).unwrap_or_default();
                "Infinite FP".create_toggle_str(state)
            }
            Self::InfiniteConsumables => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::InfiniteGoods).unwrap_or_default();
                "Infinite Consumables".create_toggle_str(state)
            }
            Self::Silent => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::Silent).unwrap_or_default();
                "Silent".create_toggle_str(state)
            }
            Self::Hidden => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::Hidden).unwrap_or_default();
                "Hidden".create_toggle_str(state)
            }
            Self::InfiniteArrows => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::InfiniteArrows).unwrap_or_default();
                "Infinite Arrows".create_toggle_str(state)
            }
            Self::TorrentNoDeath => {
                let state = player::torrent_ins().is_no_death().unwrap_or_default() ||
                    get_state_flag(GameStateFlags::TorrentNoDeath);
                "Torrent No Death".create_toggle_str(state)
            }
            Self::TorrentAnywhere=> {
                let state = player::is_torrent_anywhere().unwrap_or_default();
                "Torrent Anywhere".create_toggle_str(state)
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
    ];
    fn list(player_tab: &PlayerTab, player_ins: &ChrIns) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(player_tab, player_ins)).collect();
        tabs_list(items, None, &player_tab.tab, TOGGLES_IDX)
    }
}

impl Stats {
    fn to_list_item(&self, stats: &PlayerStats) -> ListItem<'_> {
        let text = match self {
            Self::Vigor => format!("{} Vigor", stats.vigor),
            Self::Mind => format!("{} Mind", stats.mind),
            Self::Endurance => format!("{} Endurance", stats.endurance),
            Self::Strength => format!("{} Strength", stats.strength),
            Self::Dexterity => format!("{} Dexterity", stats.dexterity),
            Self::Intelligence => format!("{} Intelligence", stats.intelligence),
            Self::Faith => format!("{} Faith", stats.faith),
            Self::Arcane => format!("{} Arcane", stats.arcane),
            Self::Scadutree => format!("{} Scadutree", stats.scadutree),
            Self::SpiritAsh => format!("{} Spirit Ash", stats.spirit_ash),
            Self::RuneLevel => format!("{} Rune Memory", stats.rune_memory),
            Self::RuneMem => format!("{} Rune Memory", stats.rune_memory),
        };
        ListItem::from(text)
    }
    fn set_input(&self) {
        send_input_event!(text, app, {
            if let Ok(v) = text.parse() {
                let idx = app.elden_ring.player.tab.lists_states[STATS_IDX].selected().unwrap_or_default();
                let stat  = &Stats::array(app.elden_ring.er_info.dlc)[idx];
                stat.set_stat(v).send_error();
            }
        });
    }

    pub fn set_stat(&self, val: i32) -> Result<()> {
        match self {
            Self::Vigor => player::set_stat(player_game_data_offsets::VIGOR, val),
            Self::Mind => player::set_stat(player_game_data_offsets::MIND, val),
            Self::Endurance => player::set_stat(player_game_data_offsets::ENDURANCE, val),
            Self::Strength => player::set_stat(player_game_data_offsets::STRENGTH, val),
            Self::Dexterity => player::set_stat(player_game_data_offsets::DEXTERITY, val),
            Self::Intelligence => player::set_stat(player_game_data_offsets::INTELLIGENCE, val),
            Self::Faith => player::set_stat(player_game_data_offsets::FAITH, val),
            Self::Arcane => player::set_stat(player_game_data_offsets::ARCANE, val),
            Self::Scadutree => player::set_dlc_stat(player_game_data_offsets::SCADUTREE, val as u8),
            Self::SpiritAsh => player::set_dlc_stat(player_game_data_offsets::SPIRIT_ASH, val as u8),
            _ => Ok(()),
        }
    }
    fn increment_stat(&self, stats: &PlayerStats, val: i32) -> Result<()> {
        match self {
            Self::Vigor => self.set_stat(stats.vigor + val),
            Self::Mind => self.set_stat(stats.mind + val),
            Self::Endurance => self.set_stat(stats.endurance + val),
            Self::Strength => self.set_stat(stats.strength + val),
            Self::Dexterity => self.set_stat(stats.dexterity + val),
            Self::Intelligence => self.set_stat(stats.intelligence + val),
            Self::Faith => self.set_stat(stats.faith + val),
            Self::Arcane => self.set_stat(stats.arcane + val),
            Self::Scadutree => self.set_stat(stats.scadutree as i32 + val),
            Self::SpiritAsh => self.set_stat(stats.spirit_ash as i32 + val),
            _ => Ok(()),
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
    pub fn array(dlc: bool) -> &'static [Stats] {
        if dlc { Self::NO_DLC_ARRAY } else { Self::ARRAY }
    }
    fn list(player_tab: &PlayerTab, dlc: bool) -> List<'static> {
        let array = Self::array(dlc);
        let items: Vec<ListItem> = array.iter().map(|i| i.to_list_item(&player_tab.stats)).collect();
        tabs_list(items, Some("Stats"), &player_tab.tab, STATS_IDX)
    }
}
