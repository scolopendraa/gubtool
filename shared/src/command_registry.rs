use {
    crate::{
        act_array::ActArray,
        command::{Command, StatCommand, ToggleCommand, UnitCommand, ValCmd},
    },
    anyhow::{anyhow, bail},
};

pub struct CommandRegistration {
    pub name:    &'static str,
    pub command: &'static Command,
}

pub struct CommandRegistry {
    register: &'static linkme::DistributedSlice<[CommandRegistration]>,
}

impl CommandRegistry {
    pub const fn new(register: &'static linkme::DistributedSlice<[CommandRegistration]>) -> Self {
        Self {
            register,
        }
    }

    pub fn get_command(&self, command_name: &str) -> Option<&'static Command> {
        self.register
            .iter()
            .find(|registration| registration.name == command_name)
            .map(|registration| registration.command)
    }

    pub fn print_commands(&self) {
        self.register.iter().for_each(|r| println!("{}", r.name));
    }
}

pub enum CommandAction {
    Toggle {
        command: &'static dyn ToggleCommand,
        action:  ToggleAction,
    },
    Value {
        command: ValCmd,
        value:   String,
    },
    Stat {
        command: &'static dyn StatCommand,
        value:   String,
    },
    Unit {
        command: &'static dyn UnitCommand,
    },
}

pub enum ToggleAction {
    Toggle,
    SetOn,
    SetOff,
}

impl CommandAction {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::Toggle {
                command,
                action,
            } => {
                match action {
                    ToggleAction::Toggle => command.toggle(),
                    ToggleAction::SetOn => command.set(true),
                    ToggleAction::SetOff => command.set(false),
                }
            }
            Self::Value {
                command,
                value,
            } => {
                match command {
                    ValCmd::I32(v) => v.set(value.parse::<i32>()?),
                    ValCmd::F32(v) => v.set(value.parse::<f32>()?),
                    ValCmd::U8(v) => v.set(value.parse::<u8>()?),
                    ValCmd::U32(v) => v.set(value.parse::<u32>()?),
                    ValCmd::U64(v) => v.set(value.parse::<u64>()?),
                    ValCmd::ActArray(v) => v.set(value.parse::<ActArray>()?),
                }
            }
            Self::Stat {
                command,
                value,
            } => {
                let val = value.parse::<u32>()?;
                command.set(val)
            }
            Self::Unit {
                command,
            } => command.execute(),
        }
    }

    pub fn get(command: &'static Command, arg: Option<String>) -> anyhow::Result<Self> {
        let c = match command {
            Command::Toggle(v) => {
                let action = match arg.as_deref() {
                    Some("on") => ToggleAction::SetOn,
                    Some("off") => ToggleAction::SetOff,
                    Some("toggle") => ToggleAction::Toggle,
                    Some(value) => {
                        bail!("invalid argument '{value}'; expected 'on', 'off', or 'toggle'")
                    }
                    None => bail!("missing argument; expected 'on', 'off', or 'toggle'"),
                };
                Self::Toggle {
                    command: *v,
                    action,
                }
            }
            Command::Unit(v) => {
                Self::Unit {
                    command: *v,
                }
            }
            Command::Value(v) => {
                let value = arg.ok_or(anyhow!("missing argument"))?;
                Self::Value {
                    command: *v,
                    value,
                }
            }
            Command::Stat(v) => {
                let value = arg.ok_or(anyhow!("missing argument"))?;
                Self::Stat {
                    command: *v,
                    value,
                }
            }
            _ => panic!("matched invalid command"),
        };
        Ok(c)
    }
}
