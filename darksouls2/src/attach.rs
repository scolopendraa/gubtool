use {
    crate::{event, player, target, utility},
    config::{
        attach::{AttachEntry, apply_attach_entries},
        impl_attach_field_bool,
    },
    gubtool_core::{attached, game_version::Game},
    shared::command::ToggleCommand,
    std::time::Duration,
};

pub async fn attach() -> anyhow::Result<()> {
    crate::init();

    let time_to_wait = 6.0 - attached::uptime();

    if time_to_wait > 0.1 {
        tokio::time::sleep(Duration::from_secs_f64(time_to_wait)).await;
    }

    if attached::game() != Ok(Game::DarkSouls2) {
        return Ok(());
    }

    target::SaveTargetHook.set(true)?;
    target::ActHook.set(true)?;
    apply_attach_entries(&ATTACH_ENTRIES)?;
    Ok(())
}

impl_attach_field_bool!(NoDeath, dark_souls_2, no_death, player::NoDeath);
impl_attach_field_bool!(NoDamage, dark_souls_2, no_damage, player::NoDamage);
impl_attach_field_bool!(InfinitePoise, dark_souls_2, infinite_poise, player::InfinitePoise);
impl_attach_field_bool!(InfiniteStamina, dark_souls_2, infinite_stamina, player::InfiniteStamina);
impl_attach_field_bool!(
    InfiniteDurability,
    dark_souls_2,
    infinite_durability,
    player::InfiniteDurability
);
impl_attach_field_bool!(
    InfiniteConsumables,
    dark_souls_2,
    infinite_consumables,
    player::InfiniteConsumables
);
impl_attach_field_bool!(NoHollowing, dark_souls_2, no_hollowing, player::NoHollowing);
impl_attach_field_bool!(NoSoulLoss, dark_souls_2, no_soul_loss, player::NoSoulLoss);
impl_attach_field_bool!(Hidden, dark_souls_2, hidden, player::Hidden);
impl_attach_field_bool!(Silent, dark_souls_2, silent, player::Silent);
impl_attach_field_bool!(SkipCredits, dark_souls_2, skip_credits, utility::SkipCredits);
impl_attach_field_bool!(FastQuitout, dark_souls_2, fast_quitout, utility::FastQuitout);
impl_attach_field_bool!(DisableRoll, dark_souls_2, disable_roll, utility::DisableRoll);
impl_attach_field_bool!(DisableBackstep, dark_souls_2, disable_backstep, utility::DisableBackstep);
impl_attach_field_bool!(
    SkipIvoryKingGauntlet,
    dark_souls_2,
    skip_ivory_king_gauntlet,
    event::SkipIvoryKingGauntlet
);
impl_attach_field_bool!(
    DisableLoyceKnights,
    dark_souls_2,
    disable_loyce_knights,
    event::DisableLoyceKnights
);
impl_attach_field_bool!(StartEventLogger, dark_souls_2, start_event_logger, event::EventLogHook);

const ATTACH_ENTRIES: [&dyn AttachEntry; 17] = [
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
