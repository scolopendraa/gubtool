pub mod attach_config_error;
pub mod ds2_attach;
pub mod er_attach;

use {
    crate::{
        Config,
        attach::{
            attach_config_error::ApplyAttachError,
            ds2_attach::Ds2AttachConfig,
            er_attach::ErAttachConfig,
        },
    },
    gubtool_core::appdata::{AppDataError, app_data_dir, log_error},
    serde::{Deserialize, Serialize},
    std::{
        fs,
        path::PathBuf,
        sync::{LazyLock, RwLock, RwLockReadGuard},
    },
};

pub(crate) static CONFIG: LazyLock<RwLock<AttachConfig>> = LazyLock::new(|| {
    let config = match AttachConfig::read() {
        Ok(c) => c,
        Err(_) => {
            let c = AttachConfig::default();
            c.write().unwrap();
            c
        }
    };
    RwLock::new(config)
});

pub fn read_config() -> RwLockReadGuard<'static, AttachConfig> {
    CONFIG.read().unwrap()
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AttachConfig {
    #[serde(rename = "dark_souls_2")]
    pub dark_souls_2: Ds2AttachConfig,

    #[serde(rename = "elden_ring")]
    pub elden_ring: ErAttachConfig,
}

impl Config for AttachConfig {
    fn get_path() -> Result<PathBuf, AppDataError> {
        let appdata_dir = app_data_dir()?;
        Ok(appdata_dir.join("attach_options.toml"))
    }

    fn read() -> Result<Self, AppDataError> {
        let config_path = Self::get_path()?;
        let contents = fs::read_to_string(config_path)?;
        let preferences: AttachConfig = toml::from_str(&contents)?;
        Ok(preferences)
    }

    fn write(&self) -> Result<(), AppDataError> {
        let path = Self::get_path()?;
        let toml = toml::to_string(self)?;
        fs::write(path, toml)?;
        Ok(())
    }

    fn update<F>(modifier: F) -> Result<(), AppDataError>
    where F: FnOnce(&mut AttachConfig) {
        let mut toml = Self::read().unwrap_or_default();
        modifier(&mut toml);
        toml.write()
    }
}

pub fn apply_attach_entries(attach_entries: &[&dyn AttachEntry]) -> Result<(), ApplyAttachError> {
    let mut errors = Vec::new();

    for entry in attach_entries {
        if let Err(err) = entry.apply() {
            errors.push(err);
        }
    }
    let len = errors.len();
    for err in errors {
        let _ = log_error(&err);
    }
    if len > 0 {
        return Err(ApplyAttachError {
            error_count: len,
        });
    }
    Ok(())
}

pub trait AttachEntry: Sync {
    fn apply(&self) -> anyhow::Result<()>;
}

#[macro_export]
macro_rules! impl_attach_field_bool {
    ($struct_name:ident, $module:ident, $game:ident) => {
        shared::declare_command!($struct_name);

        impl shared::command::ToggleCommand for $struct_name {
            fn is(&self) -> gubtool_core::sys::sys_error::SysResult<bool> {
                Ok(config::attach::read_config().$game.$struct_name)
            }
            fn set(&self, state: bool) -> anyhow::Result<()> {
                <config::attach::AttachConfig as config::Config>::update(|c| {
                    c.$game.$struct_name = state;
                })?;
                Ok(())
            }
            fn key(&self) -> Option<&'static str> {
                None
            }
        }

        impl config::attach::AttachEntry for $struct_name {
            fn apply(&self) -> anyhow::Result<()> {
                if shared::command::ToggleCommand::is(self).unwrap() {
                    shared::command::ToggleCommand::set(&$module::$struct_name, true)
                } else {
                    Ok(())
                }
            }
        }

        $crate::register_attach_entry!($game, $struct_name);
    };
}

#[macro_export]
macro_rules! impl_attach_field_option {
    ($struct_name:ident, $module:ident, $game:ident, $type:ty) => {
        shared::declare_command!($struct_name);

        impl shared::command::OptionCommand<$type> for $struct_name {
            fn get(&self) -> Option<$type> {
                config::attach::read_config().$game.$struct_name
            }
            fn set(&self, val: Option<$type>) -> anyhow::Result<()> {
                <config::attach::AttachConfig as config::Config>::update(|c| {
                    c.$game.$struct_name = val;
                })?;
                Ok(())
            }
        }

        impl config::attach::AttachEntry for $struct_name {
            fn apply(&self) -> anyhow::Result<()> {
                if let Some(val) = shared::command::OptionCommand::get(self) {
                    shared::command::ValueCommand::<$type>::set(&$module::$struct_name, val)
                } else {
                    Ok(())
                }
            }
        }

        $crate::register_attach_entry!($game, $struct_name);
    };
}

#[macro_export]
macro_rules! register_attach_entry {
    (dark_souls_2, $struct_name:ident) => {
        const _: () = {
            #[linkme::distributed_slice(DARK_SOULS_2_ATTACH_ENTRIES)]
            static ENTRY: &'static dyn config::attach::AttachEntry = &$struct_name;
        };
    };

    (elden_ring, $struct_name:ident) => {
        const _: () = {
            #[linkme::distributed_slice(ELDEN_RING_ATTACH_ENTRIES)]
            static ENTRY: &'static dyn config::attach::AttachEntry = &$struct_name;
        };
    };
}
