#[macro_export]
macro_rules! declare_command {
    ($struct_name:ident $(=> $display_name:expr)?) => {
        pub struct $struct_name;

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                shared::declare_command!(@display f, $struct_name $(, $display_name)?)
            }
        }
    };

    (@display $f:ident, $struct_name:ident, $display_name:expr) => {
        write!($f, "{}", $display_name)
    };

    (@display $f:ident, $struct_name:ident) => {{
        let raw_name = stringify!($struct_name);
        let formatted = $crate::parse_input::title_case_from_pascal_case(&raw_name);

        write!($f, "{}", formatted)
    }};
}

#[macro_export]
macro_rules! toggle_command {
    (
        $struct_name:ident
        $((
            display = $display_name:expr
        ))?
        $((
            cli_name = $cli_name:literal
        ))?
        {
            is: $is:block
            set($state:ident): $set:block
        }
    ) => {
        $crate::declare_command!($struct_name $(=> $display_name)?);

        impl ToggleCommand for $struct_name {
            fn is(&self) -> SysResult<bool> $is

            fn set(&self, $state: bool) -> anyhow::Result<()> {
                crate::mem::ensure_game()?;
                $set
            }
            fn key(&self) -> Option<&'static str> {
                Some($crate::command_key_str!($struct_name $(, $cli_name)?))
            }
        }

        crate::link_command!(
            $crate::command::Command::Toggle(&$struct_name),
            $struct_name
            $(, $cli_name)?
        );
    };
}

#[macro_export]
macro_rules! unit_command {
    (
        $struct_name:ident
        $((
            display = $display_name:expr
        ))?
        $((
            cli_name = $cli_name:literal
        ))?
        $execute:block
    ) => {
        $crate::declare_command!($struct_name $(=> $display_name)?);

        impl UnitCommand for $struct_name {
            fn execute(&self) -> anyhow::Result<()> {
                crate::mem::ensure_game()?;
                $execute
            }
            fn key(&self) -> Option<&'static str> {
                Some($crate::command_key_str!($struct_name $(, $cli_name)?))
            }
        }

        crate::link_command!(
            $crate::command::Command::Unit(&$struct_name),
            $struct_name
            $(, $cli_name)?
        );
    };
}

#[macro_export]
macro_rules! value_command {
    (
        $struct_name:ident,
        $t: tt
        $((
            display = $display_name:expr
        ))?
        $((
            cli_name = $cli_name:literal
        ))?
        {
            get: $get:block
            set($val:ident): $set:block
        }
    ) => {
        $crate::declare_command!($struct_name $(=> $display_name)?);

        impl ValueCommand<$t> for $struct_name {
            fn get(&self) -> SysResult<$t> $get
            fn set(&self, $val: $t) -> anyhow::Result<()> {
                crate::mem::ensure_game()?;
                $set
            }
            fn key(&self) -> Option<&'static str> {
                Some($crate::command_key_str!($struct_name $(, $cli_name)?))
            }
        }

        crate::link_command!(
            $crate::command::Command::Value($crate::val_cmd!($t, $struct_name)),
            $struct_name
            $(, $cli_name)?
        );
    };
}

#[macro_export]
macro_rules! value_command_set {
    (
        $struct_name:ident,
        $t: tt
        $((
            display = $display_name:expr
        ))?
        $((
            cli_name = $cli_name:literal
        ))?
        {
            set($val:ident): $set:block
        }
    ) => {
        $crate::declare_command!($struct_name $(=> $display_name)?);

        impl ValueCommand<$t> for $struct_name {
            fn get(&self) -> SysResult<$t> {
                unreachable!("no getter for {}", $struct_name)
            }
            fn can_get(&self) -> bool {
                false
            }
            fn set(&self, $val: $t) -> anyhow::Result<()> {
                crate::mem::ensure_game()?;
                $set
            }
            fn key(&self) -> Option<&'static str> {
                Some($crate::command_key_str!($struct_name $(, $cli_name)?))
            }
        }

        crate::link_command!(
            $crate::command::Command::Value($crate::val_cmd!($t, $struct_name)),
            $struct_name
            $(, $cli_name)?
        );
    };
}

#[macro_export]
macro_rules! val_cmd {
    (i32, $struct_name:ident) => {
        $crate::command::ValCmd::I32(&$struct_name)
    };

    (f32, $struct_name:ident) => {
        $crate::command::ValCmd::F32(&$struct_name)
    };

    (u8, $struct_name:ident) => {
        $crate::command::ValCmd::U8(&$struct_name)
    };

    (u32, $struct_name:ident) => {
        $crate::command::ValCmd::U32(&$struct_name)
    };

    (u64, $struct_name:ident) => {
        $crate::command::ValCmd::U64(&$struct_name)
    };

    (ActArray, $struct_name:ident) => {
        $crate::command::ValCmd::ActArray(&$struct_name)
    };
}

#[macro_export]
macro_rules! command_key_str {
    ($struct_name:ident) => {
        $crate::convert_ascii_case!(kebab, stringify!($struct_name))
    };
    ($struct_name:ident, $explicit_name:literal) => {
        $explicit_name
    };
}

#[macro_export]
macro_rules! link_command {
    ($registry_name:ident, $struct_path:expr, $struct_name:ident) => {
        const _: () = {
            #[linkme::distributed_slice(crate::$registry_name)]
            static ENTRY: $crate::command_registry::CommandRegistration =
                $crate::command_registry::CommandRegistration {
                    name:    $crate::convert_ascii_case!(kebab, stringify!($struct_name)),
                    command: &$struct_path,
                };
        };
    };

    ($registry_name:ident, $struct_path:expr, $struct_name:ident, $cli_name:expr) => {
        const _: () = {
            #[linkme::distributed_slice(crate::$registry_name)]
            static ENTRY: $crate::command_registry::CommandRegistration =
                $crate::command_registry::CommandRegistration {
                    name:    $cli_name,
                    command: &$struct_path,
                };
        };
    };
}
