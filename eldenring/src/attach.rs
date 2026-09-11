use {
    crate::{player, target, utility},
    config::attach::{AttachEntry, apply_attach_entries},
    gubtool_core::{attached, game_version::Game},
    shared::command::ToggleCommand,
    std::time::Duration,
};

pub async fn attach() -> anyhow::Result<()> {
    crate::init();

    let time_to_wait = 5.0 - attached::uptime();

    if time_to_wait > 0.1 {
        tokio::time::sleep(Duration::from_secs_f64(time_to_wait)).await;
    }

    if !matches!(attached::game(), Ok(Game::EldenRing)) {
        return Ok(());
    }

    target::SaveTargetHook.set(true)?;
    apply_attach_entries(&ELDEN_RING_ATTACH_ENTRIES)?;
    Ok(())
}

macro_rules! impl_attach_field_bool {
    ($struct_name:ident, $module_name:ident) => {
        config::impl_attach_field_bool!($struct_name, $module_name, elden_ring);
    };
}

macro_rules! impl_attach_field_option {
    ($struct_name:ident, $module_name:ident, $t:ty) => {
        config::impl_attach_field_option!($struct_name, $module_name, elden_ring, $t);
    };
}

impl_attach_field_bool!(NoDeath, player);
impl_attach_field_bool!(NoDamage, player);
impl_attach_field_bool!(InfinitePoise, player);
impl_attach_field_bool!(OneShot, player);
impl_attach_field_bool!(RuneArc, player);
impl_attach_field_bool!(SetRfbsOnLoad, player);
impl_attach_field_bool!(NoRuneLossOnDeath, player);
impl_attach_field_bool!(NoTimePassOnDeath, player);
impl_attach_field_bool!(Hidden, player);
impl_attach_field_bool!(Silent, player);
impl_attach_field_bool!(InfiniteStamina, player);
impl_attach_field_bool!(InfiniteFp, player);
impl_attach_field_bool!(InfiniteConsumables, player);
impl_attach_field_bool!(InfiniteArrows, player);
impl_attach_field_bool!(TorrentAnywhere, player);
impl_attach_field_bool!(TorrentNoDeath, player);
impl_attach_field_option!(FpsCap, utility, f32);
impl_attach_field_option!(GameSpeed, utility, f32);
impl_attach_field_bool!(DisableLogos, utility);
impl_attach_field_bool!(MuteMusic, utility);
impl_attach_field_bool!(DisableAreaWelcomeMessage, utility);
impl_attach_field_bool!(StutterFix, utility);
impl_attach_field_bool!(MapInCombat, utility);
impl_attach_field_bool!(TravelInDungeons, utility);
impl_attach_field_bool!(DrawHitboxes, utility);
impl_attach_field_bool!(ShowAllGraces, utility);
impl_attach_field_bool!(ShowAllMaps, utility);
impl_attach_field_bool!(DisableRoll, utility);
impl_attach_field_bool!(DisableJump, utility);
impl_attach_field_bool!(DisableBackstep, utility);

#[linkme::distributed_slice]
static ELDEN_RING_ATTACH_ENTRIES: [&'static dyn AttachEntry];
