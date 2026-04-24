use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem, ListState}
};
use crate::{
    core::{
        chr_ins::ChrInsExt,
        game_state::{self, GameStateFlags, get_state_flag},
        player::{self, PlayerStats, is_chr_dbg_flag, torrent_ins},
        event,
    },
    offsets::{
        chr_dbg_flags::ChrDbgOffsets,
        game_data_man::player_game_data_offsets
    },
    tui::{
        app::App,
        input_handler::InputField,
        tabs::TabState,
        ResultExt, StrExt, list,
    },
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
        let mut sizes = vec![0; 3];
        sizes[TOGGLES_IDX] = TogglesItems::ARRAY.len();
        sizes[ACTIONS_IDX] = ActionsItems::ARRAY.len();
        sizes[STATS_IDX] = Stats::ARRAY.len();
        PlayerTab {
            tab: TabState {
                lists: vec![ListState::default().with_selected(Some(0)); 3],
                list_sizes: sizes,
                ..TabState::default()
            },
            stats: PlayerStats::new(),
            hp: 100,
            runes: 10000,
        }
    }
}

impl ActionsItems {
    fn execute(&self, app: &mut App) {
        let player_ins = &app.player_ins;
        let tx = app.sender.clone();
        match self {
            Self::SetHealth => player_ins.set_hp(app.player.hp).send_error(tx),
            Self::Die => player_ins.set_hp(0).send_error(tx),
            Self::Rest => event::rest().send_error(tx),
            Self::GiveRunes => player::give_runes(app.player.runes).send_error(tx),
            Self::AnimationSpeed => app.open_input(InputField::PlayerSpeed),
        }
    }
    fn set_input(&self, app: &mut App) {
        match self {
            Self::SetHealth => app.open_input(InputField::PlayerHp),
            Self::GiveRunes => app.open_input(InputField::GiveRunes),
            Self::AnimationSpeed => app.open_input(InputField::PlayerSpeed),
            _ => (),
        }
    }
    fn to_list_item(&self, app: &App) -> ListItem<'static> {
        let text = match self {
            Self::SetHealth => {
                format!("Set Health ({})", app.player.hp)
            }
            Self::Die => {
                "Die".to_string()
            }
            Self::Rest => {
                "Rest".to_string()
            }
            Self::GiveRunes => {
                format!("Give Runes ({})", app.player.runes)
            }
            Self::AnimationSpeed => {
                format!("Animation Speed: {}",
                    app.player_ins.get_animation_speed().unwrap_or_default())
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
    fn list(app: &App) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(app)).collect();
        list(items, None, &app.player.tab, ACTIONS_IDX)
    }
}

impl TogglesItems {
    fn execute(&self, app: &App) {
        let tx = app.sender.clone();
        match self {
            Self::NoDeath => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, new_state).send_error(tx);
            }
            Self::NoDamage => {
                let new_state = !(app.player_ins.is_no_damage().unwrap_or_default() ||
                    get_state_flag(GameStateFlags::PlayerNoDamage).unwrap_or_default());
                game_state::set_state_flag(GameStateFlags::PlayerNoDamage, new_state).send_error(tx.clone());
                app.player_ins.set_no_damage(new_state).ok();
            }
            Self::SetRfbsOnLoad => {
                let new_state = !game_state::get_state_flag(GameStateFlags::Rfbs).unwrap_or_default();
                game_state::set_state_flag(GameStateFlags::Rfbs, new_state).send_error(tx);
            }
            Self::InfinitePoise => {
                let new_state = !player::is_infinite_poise().unwrap_or_default();
                player::set_infinite_poise(new_state).send_error(tx);
            }
            Self::OneShot => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::OneShot).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::OneShot, new_state).send_error(tx);
            }
            Self::RuneArc => {
                let new_state = !(app.player.stats.rune_arc ||
                    get_state_flag(GameStateFlags::RuneArc).unwrap_or_default());
                game_state::set_state_flag(GameStateFlags::RuneArc, new_state).send_error(tx);
                player::set_rune_arc(new_state).ok();
            }
            Self::InfiniteStamina => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteStam).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteStam , new_state).send_error(tx);
            }
            Self::InfiniteFp => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteFp).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteFp, new_state).send_error(tx);
            }
            Self::InfiniteConsumables => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteGoods).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteGoods, new_state).send_error(tx);
            }
            Self::Hidden => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::Hidden).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::Hidden, new_state).send_error(tx);
            }
            Self::Silent => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::Silent).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::Silent, new_state).send_error(tx);
            }
            Self::InfiniteArrows => {
                let new_state = !is_chr_dbg_flag(ChrDbgOffsets::InfiniteArrows).unwrap_or_default();
                player::set_chr_dbg_flag(ChrDbgOffsets::InfiniteArrows, new_state).send_error(tx);
            }
            Self::TorrentNoDeath => {
                let new_state = !game_state::get_state_flag(GameStateFlags::TorrentNoDeath).unwrap_or_default();
                game_state::set_state_flag(GameStateFlags::TorrentNoDeath, new_state).send_error(tx.clone());
                let torrent_ins = torrent_ins();
                torrent_ins.set_no_death(!torrent_ins.is_no_death().unwrap_or_default()).ok();
            }
            Self::TorrentAnywhere => {
                let new_state = !player::is_torrent_anywhere().unwrap_or_default();
                player::set_torrent_anywhere(new_state).send_error(tx);
            }
        }
    }
    fn to_list_item(&self, app: &App) -> ListItem<'_> {
        let text = match self {
            Self::NoDeath => {
                let state = player::is_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath).unwrap_or_default();
                "No Death".create_toggle_str(state)
            }
            Self::NoDamage => {
                let state = app.player_ins.is_no_damage().unwrap_or_default() ||
                    get_state_flag(GameStateFlags::PlayerNoDamage).unwrap_or_default();
                "No Damage".create_toggle_str(state)
            }
            Self::SetRfbsOnLoad => {
                let state = get_state_flag(GameStateFlags::Rfbs).unwrap_or_default();
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
                let state = app.player.stats.rune_arc ||
                    get_state_flag(GameStateFlags::RuneArc).unwrap_or_default();
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
                    get_state_flag(GameStateFlags::TorrentNoDeath).unwrap_or_default();
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
    fn list(app: &App) -> List<'static> {
        let items: Vec<ListItem> = Self::ARRAY.iter().map(|i| i.to_list_item(app)).collect();
        list(items, None, &app.player.tab, TOGGLES_IDX)
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
    fn set_input(&self, app: &mut App) {
        app.open_input(InputField::PlayerStat)
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
    pub fn array(app: &App) -> &'static [Stats] {
        if app.attached && !app.game_state.dlc { Self::NO_DLC_ARRAY } else { Self::ARRAY }
    }
    fn list(app: &App) -> List<'static> {
        let array = Self::array(app);
        let items: Vec<ListItem> = array.iter().map(|i| i.to_list_item(&app.player.stats)).collect();
        list(items, Some("Stats"), &app.player.tab, STATS_IDX)
    }
}

pub fn draw(frame: &mut Frame, app: &mut App, layout: Rect) {
    if app.game_state.loaded {
        app.player.stats.update().ok();
    } else {
        app.player.stats = PlayerStats::default();
    }

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
        ActionsItems::list(app),
        layout[ACTIONS_IDX],
        &mut app.player.tab.lists[ACTIONS_IDX],
    );
    frame.render_stateful_widget(
        TogglesItems::list(app),
        layout[TOGGLES_IDX],
        &mut app.player.tab.lists[TOGGLES_IDX]
        );
    frame.render_stateful_widget(
        Stats::list(app),
        layout[STATS_IDX],
        &mut app.player.tab.lists[STATS_IDX]
    );
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    app.player.tab.handle_keys(key);
    match key.code {
        KeyCode::Char('s') => handle_input(app),
        KeyCode::Enter => handle_enter(app),
        _ => (),
    }

    let tab = &app.player.tab;
    if tab.current_list == STATS_IDX &&
    let Some(selected_idx) = tab.lists[STATS_IDX].selected() {
        let tx = app.sender.clone();
        match key.code {
            KeyCode::Char('h') => {
                Stats::array(app)[selected_idx]
                    .increment_stat(&app.player.stats, -1)
                    .send_error(tx);
            }
            KeyCode::Char('l') => {
                Stats::array(app)[selected_idx]
                    .increment_stat(&app.player.stats, 1)
                    .send_error(tx);
            }
            _ => (),
        }
    }
}

fn handle_input(app: &mut App) {
    let tab = &app.player.tab;
    let current_list = tab.current_list;
    if let Some(selected_index) = tab.lists[current_list].selected() {
        match current_list {
            ACTIONS_IDX => ActionsItems::ARRAY[selected_index].set_input(app),
            STATS_IDX => Stats::ARRAY[selected_index].set_input(app),
            _ => (),
        }
    }
}

fn handle_enter(app: &mut App) {
    let tab = &app.player.tab;
    let current_list = tab.current_list;
    if let Some(selected_index) = tab.lists[current_list].selected() {
        match current_list {
            ACTIONS_IDX => ActionsItems::ARRAY[selected_index].execute(app),
            TOGGLES_IDX => TogglesItems::ARRAY[selected_index].execute(app),
            STATS_IDX => Stats::ARRAY[selected_index].set_input(app),
            _ => (),
        }
    }
}
