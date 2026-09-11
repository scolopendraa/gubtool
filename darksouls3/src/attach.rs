use {
    crate::{target, utility}, config::attach::{AttachEntry, apply_attach_entries}, gubtool_core::{
        attached::{self},
        game_version::Game,
    }, shared::command::ToggleCommand, std::time::Duration,
};

pub async fn attach() -> anyhow::Result<()> {
    crate::init();

    let time_to_wait = 5.0 - attached::uptime();

    if time_to_wait > 0.1 {
        tokio::time::sleep(Duration::from_secs_f64(time_to_wait)).await;
    }

    if !matches!(attached::game(), Ok(Game::DarkSouls3)) {
        return Ok(());
    }

    let _ = utility::disable_logos();

    target::SaveTargetHook.set(true)?;
    apply_attach_entries(&DARK_SOULS_3_ATTACH_ENTRIES)?;
    Ok(())
}

macro_rules! _impl_attach_field_bool {
    ($struct_name:ident, $module_name:ident) => {
        config::impl_attach_field_bool!($struct_name, $module_name, dark_souls_3);
    };
}

#[linkme::distributed_slice]
static DARK_SOULS_3_ATTACH_ENTRIES: [&'static dyn AttachEntry];
