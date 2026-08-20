use {
    crate::{act_array::ActArray, parse_input::ParseInput},
    gubtool_core::sys::sys_error::SysResult,
    std::fmt::Display,
};

pub enum Command {
    Unit(&'static dyn UnitCommand),
    Toggle(&'static dyn ToggleCommand),
    Stat(&'static dyn StatCommand),
    Empty(&'static dyn EmptyCommand),
    Value(ValCmd),
    Option(OptCmd),
}

pub enum ValCmd {
    I32(&'static dyn ValueCommand<i32>),
    F32(&'static dyn ValueCommand<f32>),
    U8(&'static dyn ValueCommand<u8>),
    U32(&'static dyn ValueCommand<u32>),
    U64(&'static dyn ValueCommand<u64>),
    ActArray(&'static dyn ValueCommand<ActArray>),
}

pub enum OptCmd {
    F32(&'static dyn OptionCommand<f32>),
}

pub trait UnitCommand: Send + Sync + Display {
    fn execute(&self) -> anyhow::Result<()>;
}

pub trait ToggleCommand: Send + Sync + Display {
    fn is(&self) -> SysResult<bool>;
    fn set(&self, state: bool) -> anyhow::Result<()>;

    fn toggle(&self) -> anyhow::Result<()> {
        let new_state = !self.is().unwrap_or_default();
        self.set(new_state)
    }
}

pub trait StatCommand: Send + Sync + Display {
    fn get(&self) -> u32;
    fn set(&self, val: u32) -> anyhow::Result<()>;

    fn increment(&self, inc: i32) -> anyhow::Result<()> {
        let current = self.get();
        self.set(current.saturating_add_signed(inc))
    }
}

pub trait EmptyCommand: Send + Sync + Display {}

pub trait OptionCommand<T>: Send + Sync + Display
where T: Display + Send + 'static + ParseInput + Default
{
    fn get(&self) -> Option<T>;
    fn set(&self, val: Option<T>) -> anyhow::Result<()>;
}

pub trait ValueCommand<T>: Send + Sync + Display
where T: Display + Send + 'static + ParseInput + Default
{
    fn get(&self) -> SysResult<T>;
    fn set(&self, val: T) -> anyhow::Result<()>;

    fn can_get(&self) -> bool {
        true
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: &dyn Display = match self {
            Self::Unit(v) => v,
            Self::Toggle(v) => v,
            Self::Stat(v) => v,
            Self::Empty(v) => v,
            Self::Value(v) => v,
            Self::Option(v) => v,
        };

        value.fmt(f)
    }
}

impl Display for ValCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: &dyn Display = match self {
            Self::I32(v) => v,
            Self::F32(v) => v,
            Self::U8(v) => v,
            Self::U32(v) => v,
            Self::U64(v) => v,
            Self::ActArray(v) => v,
        };

        value.fmt(f)
    }
}

impl Display for OptCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: &dyn Display = match self {
            Self::F32(v) => v,
        };

        value.fmt(f)
    }
}
