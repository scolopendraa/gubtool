use {
    crate::{
        common::helpers::create_toggle_string,
        event::{AnyhowExt, send_success},
        input::request_input,
        panes::TableView,
        spawn_task,
    },
    gubtool_core::game_version::Game,
    ratatui::widgets::Row,
    shared::{
        command::{Command, OptCmd, OptionCommand, ValCmd, ValueCommand},
        parse_input::ParseInput,
    },
    std::fmt::Display,
};

pub fn commands_to_table_view(commands: &'static [Command]) -> TableView {
    let rows = commands
        .iter()
        .map(|c| {
            let text = match c {
                Command::Toggle(v) => create_toggle_string(&c.to_string(), v.is().unwrap_or(false)),
                Command::Unit(_) => format!("{c}"),
                Command::Stat(v) => format!("{:02} {c}", v.get()),
                Command::Empty(_) => format!("{c}"),
                Command::Value(v) => display_val_cmd(v),
                Command::Option(v) => display_opt_cmd(v),
            };
            Row::new([text])
        })
        .collect();

    TableView::new(rows)
}

pub fn execute(command: &'static Command) {
    spawn_task! {
        match command {
            Command::Toggle(v) => v.toggle().send_error(),
            Command::Unit(v) => v.execute().send_error(),
            Command::Value(v) => execute_val_cmd(v),
            Command::Option(v) => execute_opt_cmd(v),
            Command::Empty(_) => unreachable!("command should not be handled here"),
            Command::Stat(v) => {
                if let Some(val) = request_input::<u32>(None).await {
                    v.set(val).send_error();
                }
            }
        }
    }
}

fn display_val_cmd(val_cmd: &ValCmd) -> String {
    match *val_cmd {
        ValCmd::U8(v) => display_val(v),
        ValCmd::I32(v) => display_val(v),
        ValCmd::F32(v) => display_val(v),
        ValCmd::U32(v) => display_val(v),
        ValCmd::U64(v) => display_val(v),
        ValCmd::ActArray(v) => display_val(v),
    }
}

fn display_val<T>(cmd: &'static dyn ValueCommand<T>) -> String
where T: Display + Send + 'static + ParseInput + Default {
    if cmd.can_get() {
        format!("{cmd}: {:.2}", cmd.get().unwrap_or_default())
    } else {
        format!("{cmd}")
    }
}

fn display_opt_cmd(opt_cmd: &OptCmd) -> String {
    match *opt_cmd {
        OptCmd::F32(v) => display_opt(v),
    }
}

fn display_opt<T>(cmd: &'static dyn OptionCommand<T>) -> String
where T: Display + Send + 'static + ParseInput + Default {
    match cmd.get() {
        Some(v) => format!("{cmd}: {:.2}", v),
        None => format!("{cmd}:"),
    }
}

fn execute_val_cmd(val_cmd: &ValCmd) {
    match *val_cmd {
        ValCmd::U8(v) => request_and_set_val(v),
        ValCmd::I32(v) => request_and_set_val(v),
        ValCmd::F32(v) => request_and_set_val(v),
        ValCmd::U32(v) => request_and_set_val(v),
        ValCmd::U64(v) => request_and_set_val(v),
        ValCmd::ActArray(v) => request_and_set_val(v),
    }
}

fn request_and_set_val<T>(cmd: &'static dyn ValueCommand<T>)
where T: Display + Send + 'static + ParseInput + Default {
    spawn_task! {
        if let Some(val) = request_input::<T>(None).await {
            cmd.set(val).send_error();
        }
    }
}

fn execute_opt_cmd(opt_cmd: &OptCmd) {
    match *opt_cmd {
        OptCmd::F32(v) => request_and_set_opt(v),
    }
}

fn request_and_set_opt<T>(cmd: &'static dyn OptionCommand<T>)
where T: Display + Send + 'static + ParseInput + Default {
    spawn_task! {
        let val = request_input::<T>(None).await;
        cmd.set(val).send_error();
    }
}

pub fn display_cli_command(command: Command, game_screen: Game) {
    if let Some(key) = command.key() {
        let game_flag = match game_screen {
            Game::DarkSouls2 => "--ds2",
            Game::EldenRing => "--er",
        };

        let text = match command {
            Command::Unit(_) => format!("gubtool {} {}", game_flag, key),
            Command::Toggle(_) => format!("gubtool {} {} <state>", game_flag, key),
            Command::Value(_) => format!("gubtool {} {} <val>", game_flag, key),
            _ => unreachable!("key is not implemented for remaining commands"),
        };
        send_success(text);
    }
}

#[macro_export]
macro_rules! impl_tablecontroller_for_commands {
    ($name:ident, $items:ident) => {
        struct $name;

        impl crate::panes::TableController for $name {
            fn make_table_view(&self) -> crate::panes::TableView {
                crate::command::commands_to_table_view(&$items)
            }

            fn handle_keys_selected(&self, selected: usize, ctx: &mut crate::event::KeyContext) {
                if ctx.key_enter() {
                    crate::command::execute(&$items[selected]);
                    return;
                }

                match self.get_command(selected) {
                    Some(shared::command::Command::Stat(c)) => {
                        if ctx.key_char('h') || ctx.key(KeyCode::Left) {
                            crate::event::AnyhowExt::send_error(c.increment(-1));
                        }
                        if ctx.key_char('l') || ctx.key(KeyCode::Right) {
                            crate::event::AnyhowExt::send_error(c.increment(1));
                        }
                    }
                    _ => (),
                }
            }

            fn get_command(&self, selected: usize) -> Option<&shared::command::Command> {
                Some(&$items[selected])
            }
        }
    };
}
