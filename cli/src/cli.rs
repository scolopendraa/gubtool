use {
    anyhow::{Ok, ensure},
    clap::{Parser, Subcommand, ValueEnum},
    gubtool_core::{
        attached::{self, game},
        game_version::Game,
    },
    shared::command::{ToggleCommand, UnitCommand},
    std::{thread, time::Duration},
};

#[derive(Parser)]
#[command(name = "gubtool")]
#[derive(Clone, Copy)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Subcommand, Clone, Copy)]
pub enum CliCommand {
    Quitout,
    KillTarget,
    NextPhase,
    #[cfg(debug_assertions)]
    AobScan,
    #[cfg(debug_assertions)]
    AsmSizes,
    #[cfg(debug_assertions)]
    Test,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.command.is_none() {
        if let Err(e) = tui::run() {
            eprintln!("{e:?}");
        }
        return Ok(());
    }

    #[cfg(debug_assertions)]
    match cli.command.unwrap() {
        CliCommand::AsmSizes => {
            gubtool_core::sys::print_asm_sizes();
            darksouls2::utils::print_asm_sizes();
            eldenring::utils::print_asm_sizes();
            return Ok(());
        }
        CliCommand::Test => {}
        _ => (),
    }

    attached::try_auto_attach();
    ensure!(attached::is_attached(), "Game not found");

    let game = game().unwrap();
    match game {
        Game::DarkSouls2 => darksouls2::init(),
        Game::EldenRing => eldenring::init(),
    }

    #[allow(unreachable_patterns)]
    match cli.command.unwrap() {
        CliCommand::Quitout => {
            match game {
                Game::EldenRing => eldenring::utility::Quitout.execute()?,
                Game::DarkSouls2 => darksouls2::utility::Quitout.execute()?,
            }
        }
        CliCommand::KillTarget => {
            match game {
                Game::EldenRing => {
                    if !eldenring::target::SaveTargetHook.is()? {
                        eldenring::target::SaveTargetHook.set(true)?;
                        thread::sleep(Duration::from_millis(50));
                    }
                    eldenring::target::target().chr_ins()?.set_hp(0)?
                }
                Game::DarkSouls2 => {
                    if !darksouls2::target::SaveTargetHook.is()? {
                        darksouls2::target::SaveTargetHook.set(true)?;
                        thread::sleep(Duration::from_millis(50));
                    }
                    darksouls2::target::target().chr_ctrl()?.set_hp(0)?
                }
            }
        }
        CliCommand::NextPhase => {
            match game {
                Game::EldenRing => eldenring::target::next_phase()?,
                Game::DarkSouls2 => (),
            }
        }
        #[cfg(debug_assertions)]
        CliCommand::AobScan => {}
        #[cfg(debug_assertions)]
        CliCommand::Test => eldenring::target::unlock()?,
        _ => (),
    }
    Ok(())
}

#[derive(Clone, ValueEnum)]
pub enum OnOff {
    On  = 1,
    Off = 0,
}

impl From<OnOff> for bool {
    fn from(val: OnOff) -> Self {
        val as u8 != 0
    }
}
