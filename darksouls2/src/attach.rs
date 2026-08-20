use {
    crate::{event, player, target, utility},
    config::{
        attach::{AttachEntry, apply_attach_entries},
        impl_attach_field_bool,
    },
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
    let _ = utility::enable_skip_logos();

    let time_to_wait = 6.0 - attached::uptime();

    if time_to_wait > 0.1 {
        tokio::time::sleep(Duration::from_secs_f64(time_to_wait)).await;
    }

    if attached::game() != Ok(Game::DarkSouls2) {
        return Ok(());
    }

    target::SaveTargetHook.set(true)?;
    target::ActHook.set(true)?;
    apply_attach_entries(ATTACH_ENTRIES)?;
    Ok(())
}

impl_attach_field_bool!(NoDeath, player, dark_souls_2);
impl_attach_field_bool!(NoDamage, player, dark_souls_2);
impl_attach_field_bool!(InfinitePoise, player, dark_souls_2);
impl_attach_field_bool!(InfiniteStamina, player, dark_souls_2);
impl_attach_field_bool!(InfiniteDurability, player, dark_souls_2);
impl_attach_field_bool!(InfiniteConsumables, player, dark_souls_2);
impl_attach_field_bool!(NoHollowing, player, dark_souls_2);
impl_attach_field_bool!(NoSoulLoss, player, dark_souls_2);
impl_attach_field_bool!(Hidden, player, dark_souls_2);
impl_attach_field_bool!(Silent, player, dark_souls_2);
impl_attach_field_bool!(SkipCredits, utility, dark_souls_2);
impl_attach_field_bool!(FastQuitout, utility, dark_souls_2);
impl_attach_field_bool!(DisableRoll, utility, dark_souls_2);
impl_attach_field_bool!(DisableBackstep, utility, dark_souls_2);
impl_attach_field_bool!(SkipIvoryKingGauntlet, event, dark_souls_2);
impl_attach_field_bool!(DisableLoyceKnights, event, dark_souls_2);
impl_attach_field_bool!(StartEventLogger, event, dark_souls_2);

const ATTACH_ENTRIES: &[&'static dyn AttachEntry] = &[
    &NoDeath,
    &NoDamage,
    &InfinitePoise,
    &InfiniteStamina,
    &InfiniteDurability,
    &InfiniteConsumables,
    &NoHollowing,
    &NoSoulLoss,
    &Hidden,
    &Silent,
    &SkipCredits,
    &FastQuitout,
    &DisableRoll,
    &DisableBackstep,
    &SkipIvoryKingGauntlet,
    &DisableLoyceKnights,
    &StartEventLogger,
];
