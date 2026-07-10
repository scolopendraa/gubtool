use crate::{
    achievement,
    chr_ins::{ChrInsExt, chr_ins_from_handle},
    emevd,
    mem::*,
    offsets::{
        ChainReadExt, code_cave::CaveOffset, game_data_man, menu_man, module_offsets::BasePointer,
        world_chr_man,
    },
    player::{self, player_ins, torrent_ins},
    target::target_ins,
    travel,
    utility,
    utils::{is_dlc_available, is_version_dlc_compat},
};
use gubtool_core::{address::Address, slice_ops::*, sys::error::ProcResult};

#[derive(Default)]
pub struct GameStateHandler {
    pub loaded: bool,
    has_invoked_load_delayed: bool,
    has_invoked_loaded: bool,
    pub dlc: bool,
    flags: StateFlags,
}

#[derive(Default, Clone, Copy)]
pub struct StateFlags {
    pub player_no_damage: bool,
    pub rfbs: bool,
    pub title_cards: bool,
    pub rune_arc: bool,
    pub torrent_no_death: bool,
    pub stutter_fix: bool,
    pub hitboxes: bool,
    pub set_1hp: bool,
    pub no_time_change_death: bool,
    pub disable_achievements: bool,
    pub no_rune_loss: bool,
}

impl GameStateHandler {
    pub fn new() -> Self {
        let flags = StateFlags::const_default();
        Self {
            loaded: false,
            has_invoked_load_delayed: true,
            has_invoked_loaded: true,
            dlc: is_version_dlc_compat(),
            flags,
        }
    }

    pub fn poll(&mut self) -> ProcResult {
        if is_loaded() {
            if !self.has_invoked_load_delayed && self.has_invoked_loaded && is_faded_in() {
                self.on_load_delayed()?;
                self.has_invoked_load_delayed = true;
            }
            if !self.loaded {
                self.loaded = true;
                self.on_loaded()?;
                self.has_invoked_loaded = true;

                if is_new_game() {
                    self.on_new_game()?;
                }
            }
            // Cache is_player_dead() result to prevent inconsistent reads
            // across multiple features in the same poll cycle.
            let player_dead = emevd::is_player_dead().unwrap_or(false);

            // Check for death/time change prevention on every poll.
            // First detect if the player just died (sets DeathFlag),
            // then check if time needs to be restored (clears DeathFlag).
            if self.flags.no_time_change_death {
                let _ = emevd::detect_death_and_set_flag();
                let _ = emevd::check_and_restore_time_on_death();
            }
            // Apply set_1hp on every poll if enabled, but skip when player is dead
            // to avoid interfering with death animations and respawn logic.
            if self.flags.set_1hp && !player_dead {
                let _ = player::set_1hp();
            }
        } else if self.loaded {
            self.on_unloaded();
            self.has_invoked_load_delayed = false;
            self.has_invoked_loaded = false;
            self.loaded = false;
        }
        Ok(())
    }
    fn on_loaded(&mut self) -> ProcResult {
        self.flags = StateFlags::new()?;

        if self.flags.player_no_damage {
            player_ins().set_no_damage(true)?;
        }
        if self.flags.title_cards {
            emevd::disable_title_card()?;
        }
        if self.flags.rune_arc {
            player::set_rune_arc(true)?;
        }
        if self.flags.stutter_fix {
            utility::set_stutter_fix(true)?;
        }
        if self.flags.hitboxes {
            utility::draw_hitboxes(true, false)?;
        }
        if self.flags.disable_achievements {
            achievement::install_patch()?;
        }

        // Initialize time of day tracking if the no_time_change_death flag is set
        if self.flags.no_time_change_death {
            let _ = emevd::init_time_of_day();
        }

        let handle = read::<u64>(CaveOffset::LookedUpHandle)?;
        write::<u64>(CaveOffset::SavedTargetPointer, chr_ins_from_handle(handle).unwrap_or_default())?;

        self.dlc = is_dlc_available();
        Ok(())
    }

    fn on_load_delayed(&self) -> ProcResult {
        if self.flags.rfbs {
            player::set_rfbs()?;
        }
        if self.flags.torrent_no_death {
            torrent_ins().set_no_death(true)?;
        }
        if self.flags.set_1hp {
            player::set_1hp()?;
        }
        Ok(())
    }

    fn on_unloaded(&self) {
        // Clean up any warp hooks that might still be installed
        let _ = travel::cleanup_warp_hooks();
        // Reset warp hook state to prevent stale bytes on re-attach
        travel::reset_warp_hook_state();
        write::<u64>(CaveOffset::LookedUpHandle, target_ins().handle().unwrap_or_default()).ok();
        // Remove achievement patch
        let _ = achievement::uninstall_patch();
        // Clear death flag
        let _ = write::<u8>(CaveOffset::DeathFlag, 0);
    }

    fn on_new_game(&self) -> ProcResult {
        // Reset time of day tracking on new game
        if self.flags.no_time_change_death {
            let _ = emevd::init_time_of_day();
        }
        Ok(())
    }

    /// Get a reference to the current state flags.
    pub fn flags(&self) -> &StateFlags {
        &self.flags
    }
}

impl StateFlags {
    pub fn new() -> ProcResult<Self> {
        let mut flags = Self::default();
        flags.update()?;
        Ok(flags)
    }
    pub fn update(&mut self) -> ProcResult {
        let flags = read::<[u8; 0x100]>(CaveOffset::StateHandlerFlags)?;
        self.player_no_damage = read_flag_from_slice(&flags, StateFlagOffset::PlayerNoDamage)?;
        self.rfbs = read_flag_from_slice(&flags, StateFlagOffset::Rfbs)?;
        self.title_cards = read_flag_from_slice(&flags, StateFlagOffset::TitleCards)?;
        self.rune_arc = read_flag_from_slice(&flags, StateFlagOffset::RuneArc)?;
        self.torrent_no_death = read_flag_from_slice(&flags, StateFlagOffset::TorrentNoDeath)?;
        self.stutter_fix = read_flag_from_slice(&flags, StateFlagOffset::StutterFix)?;
        self.hitboxes = read_flag_from_slice(&flags, StateFlagOffset::Hitboxes)?;
        self.set_1hp = read_flag_from_slice(&flags, StateFlagOffset::Set1hp)?;
        self.no_time_change_death = read_flag_from_slice(&flags, StateFlagOffset::NoTimeChangeDeath)?;
        self.disable_achievements = read_flag_from_slice(&flags, StateFlagOffset::DisableAchievements)?;
        self.no_rune_loss = read_flag_from_slice(&flags, StateFlagOffset::NoRuneLoss)?;
        Ok(())
    }
    pub fn set(flag_offset: StateFlagOffset, state: bool) -> ProcResult {
        write::<u8>(CaveOffset::StateHandlerFlags.add_offset(flag_offset as u64), state as u8)
    }
    pub const fn const_default() -> Self {
        Self {
            player_no_damage: false,
            rfbs: false,
            title_cards: false,
            rune_arc: false,
            torrent_no_death: false,
            stutter_fix: false,
            hitboxes: false,
            set_1hp: false,
            no_time_change_death: false,
            disable_achievements: false,
            no_rune_loss: false,
        }
    }
}

#[repr(u64)]
pub enum StateFlagOffset {
    PlayerNoDamage = 0x0,
    Rfbs = 0x1,
    TitleCards = 0x2,
    RuneArc = 0x3,
    TorrentNoDeath = 0x4,
    StutterFix = 0x5,
    Hitboxes = 0x6,
    Set1hp = 0x7,
    NoTimeChangeDeath = 0x8,
    DisableAchievements = 0x9,
    NoRuneLoss = 0xA,
}

fn read_flag_from_slice(flags: &[u8; 0x100], flag_offset: StateFlagOffset) -> Result<bool, SliceError> {
    read_from_slice::<u8>(flags, flag_offset as u64).map(|val| val != 0x0)
}

pub fn is_loaded() -> bool {
    read::<u64>(BasePointer::WorldChrMan)
        .read_offset(world_chr_man::player_ins())
        .map(|val| val != 0)
        .unwrap_or_default()
}

fn is_faded_in() -> bool {
    read::<u64>(BasePointer::MenuMan)
        .add_offset(menu_man::is_fading())
        .read::<u8>()
        .map(|val| val == 0x0)
        .unwrap_or_default()
}

fn is_new_game() -> bool {
    read::<u64>(BasePointer::GameDataMan)
        .add_offset(game_data_man::IGT)
        .read::<u64>()
        .map(|val| val < 5000)
        .unwrap_or_default()
}