use {
    crate::{enemy, event, player, target, utility},
    config::attach::{AttachEntry, apply_attach_entries},
    gubtool_core::{
        attached::{self},
        game_version::Game,
    },
    shared::command::ToggleCommand,
    std::time::Duration,
};

pub async fn attach() -> anyhow::Result<()> {
    crate::init();

    tokio::time::sleep(Duration::from_secs_f64(1.0)).await;
    utility::enable_skip_logos()?;

    let time_to_wait = 6.0 - attached::uptime();

    if time_to_wait > 0.1 {
        tokio::time::sleep(Duration::from_secs_f64(time_to_wait)).await;
    }

    if !matches!(attached::game(), Ok(Game::DarkSouls2)) {
        return Ok(());
    }

    target::SaveTargetHook.set(true)?;
    enemy::install_act_logger_hook()?;

    apply_attach_entries(&DARK_SOULS_2_ATTACH_ENTRIES)?;
    Ok(())
}

macro_rules! impl_attach_field_bool {
    ($struct_name:ident, $module_name:ident) => {
        config::impl_attach_field_bool!($struct_name, $module_name, dark_souls_2);
    };
}

impl_attach_field_bool!(NoDeath, player);
impl_attach_field_bool!(NoDamage, player);
impl_attach_field_bool!(InfinitePoise, player);
impl_attach_field_bool!(InfiniteStamina, player);
impl_attach_field_bool!(InfiniteDurability, player);
impl_attach_field_bool!(InfiniteConsumables, player);
impl_attach_field_bool!(NoHollowing, player);
impl_attach_field_bool!(NoSoulLoss, player);
impl_attach_field_bool!(Hidden, player);
impl_attach_field_bool!(Silent, player);
impl_attach_field_bool!(SkipCredits, utility);
impl_attach_field_bool!(FastQuitout, utility);
impl_attach_field_bool!(DisableRoll, utility);
impl_attach_field_bool!(DisableBackstep, utility);
impl_attach_field_bool!(SkipIvoryKingGauntlet, event);
impl_attach_field_bool!(DisableLoyceKnights, event);
impl_attach_field_bool!(StartEventLogger, event);

#[linkme::distributed_slice]
static DARK_SOULS_2_ATTACH_ENTRIES: [&'static dyn AttachEntry];
