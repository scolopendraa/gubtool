use crate::{
    core::attach::{self, Game, game}, ds2, er::{
        self, chr_ins::ChrInsExt, item, offsets::chr_dbg_flags::ChrDbgOffsets, player::{self, player_ins}, resources::items, target
    }, tui::tui
};
use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::{thread, time::Duration};

#[derive(Parser)]
#[command(name = "gubtool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Quitout,
    KillTarget,
    PlayerHp { val: i32 },
    TargetHp { val: i32 },
    FreezeWorldToggle,
    LogoPatch { val: OnOff },
    NoDeath { val: OnOff },
    OneShot { val: OnOff },
    MuteMusic { val: OnOff },
    TriggerNG,
    GiveRunes { val: i64 },
    Fps { val: f32 },
    ShowGraces { val: OnOff },
    ShowMaps { val: OnOff },
    TargetInfinitePoise { val: OnOff },
    MassSpawn { val: items::Categories },
    Test,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    if cli.command.is_none() {
        tui().ok();
        return Ok(());
    }

    attach::auto_attach()?;

    match cli.command.unwrap() {
        Commands::Quitout => {
            match game() {
                Game::EldenRing => er::utility::quitout(),
                Game::DarkSoulsII => ds2::utility::quitout(),
            }
        }
        Commands::PlayerHp { val } => {
            player_ins().set_hp(val)
        }
        Commands::LogoPatch { val } => {
            er::utility::set_logo_patch(val.into())
        }
        Commands::FreezeWorldToggle => {
            er::utility::set_freeze_world(!er::utility::is_freeze_world_on()?)
        }
        Commands::NoDeath { val } => {
            player::set_chr_dbg_flag(ChrDbgOffsets::PlayerNoDeath, val.into())
        }
        Commands::OneShot { val } => {
            player::set_chr_dbg_flag(ChrDbgOffsets::OneShot, val.into())
        }
        Commands::MuteMusic { val } => {
            er::utility::mute_music(val.into())
        }
        Commands::TriggerNG => {
            er::utility::trigger_new_game()
        }
        Commands::GiveRunes { val } => {
            player::give_runes(val)
        }
        Commands::Fps { val } => {
            er::utility::set_fps_cap(val)
        }
        Commands::ShowGraces { val } => {
            er::utility::show_all_graces(val.into())
        }
        Commands::ShowMaps { val } => {
            er::utility::show_all_maps(val.into())
        }
        Commands::KillTarget => {
            if !target::is_target_hook_active().unwrap() {
                target::install_target_hook().unwrap();
                thread::sleep(Duration::from_millis(50));
            }
            target::target_ins().set_hp(0)
        }
        Commands::TargetHp { val } => {
            target::target_ins().set_hp(val)
        }
        Commands::MassSpawn { val } => {
            item::mass_spawn(val)
        }
        Commands::TargetInfinitePoise { val } => {
            match val {
                OnOff::On => target::install_stagger_hook(),
                OnOff::Off => target::uninstall_stagger_hook(),
            }
        }
        Commands::Test => {
            Ok(())
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum OnOff {
    On = 1,
    Off = 0,
}

impl From<OnOff> for bool {
    fn from(val: OnOff) -> Self {
        val as u8 != 0
    }
}
