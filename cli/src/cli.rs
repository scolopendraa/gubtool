use {
    anyhow::bail,
    clap::{Parser, Subcommand, ValueEnum},
    gubtool_core::{attached, game_version::Game},
    shared::command_registry::{CommandAction, CommandRegistry},
};

#[derive(Parser)]
#[command(name = "gubtool")]
pub struct Cli {
    #[arg(long = "er")]
    eldenring: Option<String>,

    #[arg(long = "ds2")]
    darksouls2: Option<String>,

    #[arg(long = "any")]
    any: Option<String>,

    #[command(subcommand)]
    command: Option<CliCommand>,

    arg: Option<String>,
}

#[derive(Subcommand)]
enum CliCommand {
    ListGameCommands {
        #[arg(value_enum)]
        game: GameArg,
    },
    #[cfg(debug_assertions)]
    AsmSizes,
    #[cfg(debug_assertions)]
    Test,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    attached::try_auto_attach();

    if let Ok(game) = attached::game() {
        match game {
            Game::DarkSouls2 => darksouls2::init(),
            Game::EldenRing => eldenring::init(),
        }
    }

    match (&cli.eldenring, &cli.darksouls2, &cli.any, &cli.arg, &cli.command) {
        (Some(name), None, None, arg, None) => {
            execute_command_from_registry(&eldenring::COMMAND_REGISTER, name.clone(), arg.clone())
        }

        (None, Some(name), None, arg, None) => {
            execute_command_from_registry(&darksouls2::COMMAND_REGISTER, name.clone(), arg.clone())
        }

        (None, None, Some(name), arg, None) => {
            let registry = match attached::game() {
                Ok(Game::DarkSouls2) => &darksouls2::COMMAND_REGISTER,
                Ok(Game::EldenRing) => &eldenring::COMMAND_REGISTER,
                Err(_) => bail!("not attached to any game"),
            };
            execute_command_from_registry(registry, name.clone(), arg.clone())
        }
        (None, None, None, _, Some(command)) => handle_enum_command(command),

        (None, None, None, None, None) => {
            if let Err(e) = tui::run() {
                eprintln!("{e:?}");
            }
            Ok(())
        }

        _ => bail!("invalid command"),
    }
}

fn handle_enum_command(command: &CliCommand) -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    match command {
        CliCommand::AsmSizes => {
            gubtool_core::sys::print_asm_sizes();
            darksouls2::utils::print_asm_sizes();
            eldenring::utils::print_asm_sizes();
        }
        CliCommand::ListGameCommands {
            game,
        } => {
            match game {
                GameArg::DarkSouls2 => darksouls2::COMMAND_REGISTER.print_commands(),
                GameArg::EldenRing => eldenring::COMMAND_REGISTER.print_commands(),
            }
        }
        CliCommand::Test => {}
    }
    Ok(())
}

fn execute_command_from_registry(
    registry: &'static CommandRegistry,
    command_string: String,
    arg: Option<String>,
) -> anyhow::Result<()> {
    let Some(command) = registry.get_command(&command_string) else {
        bail!("invalid command")
    };

    let action = CommandAction::get(command, arg)?;

    action.execute()
}

#[derive(Clone, ValueEnum)]
enum GameArg {
    DarkSouls2,
    EldenRing,
}
