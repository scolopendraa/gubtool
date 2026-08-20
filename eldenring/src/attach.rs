use {
    crate::{player, target, utility},
    config::{
        attach::{AttachEntry, apply_attach_entries},
        impl_attach_field_bool,
        impl_attach_field_f32,
    },
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

    if attached::game() != Ok(Game::EldenRing) {
        return Ok(());
    }

    target::SaveTargetHook.set(true)?;
    apply_attach_entries(ATTACH_ENTRIES)?;
    Ok(())
}

impl_attach_field_bool!(NoDeath, player, elden_ring);
impl_attach_field_bool!(NoDamage, player, elden_ring);
impl_attach_field_bool!(InfinitePoise, player, elden_ring);
impl_attach_field_bool!(OneShot, player, elden_ring);
impl_attach_field_bool!(RuneArc, player, elden_ring);
impl_attach_field_bool!(SetRfbsOnLoad, player, elden_ring);
impl_attach_field_bool!(NoRuneLossOnDeath, player, elden_ring);
impl_attach_field_bool!(NoTimePassOnDeath, player, elden_ring);
impl_attach_field_bool!(Hidden, player, elden_ring);
impl_attach_field_bool!(Silent, player, elden_ring);
impl_attach_field_bool!(InfiniteStamina, player, elden_ring);
impl_attach_field_bool!(InfiniteFp, player, elden_ring);
impl_attach_field_bool!(InfiniteConsumables, player, elden_ring);
impl_attach_field_bool!(InfiniteArrows, player, elden_ring);
impl_attach_field_bool!(TorrentAnywhere, player, elden_ring);
impl_attach_field_bool!(TorrentNoDeath, player, elden_ring);
impl_attach_field_f32!(FpsCap, utility, elden_ring);
impl_attach_field_f32!(GameSpeed, utility, elden_ring);
impl_attach_field_bool!(DisableLogos, utility, elden_ring);
impl_attach_field_bool!(MuteMusic, utility, elden_ring);
impl_attach_field_bool!(DisableAreaWelcomeMessage, utility, elden_ring);
impl_attach_field_bool!(StutterFix, utility, elden_ring);
impl_attach_field_bool!(MapInCombat, utility, elden_ring);
impl_attach_field_bool!(TravelInDungeons, utility, elden_ring);
impl_attach_field_bool!(DrawHitboxes, utility, elden_ring);
impl_attach_field_bool!(ShowAllGraces, utility, elden_ring);
impl_attach_field_bool!(ShowAllMaps, utility, elden_ring);
impl_attach_field_bool!(DisableRoll, utility, elden_ring);
impl_attach_field_bool!(DisableJump, utility, elden_ring);
impl_attach_field_bool!(DisableBackstep, utility, elden_ring);

const ATTACH_ENTRIES: &[&'static dyn AttachEntry] = &[
    &NoDeath,
    &NoDamage,
    &InfinitePoise,
    &OneShot,
    &RuneArc,
    &Hidden,
    &Silent,
    &InfiniteStamina,
    &InfiniteFp,
    &InfiniteConsumables,
    &InfiniteArrows,
    &SetRfbsOnLoad,
    &NoRuneLossOnDeath,
    &NoTimePassOnDeath,
    &TorrentAnywhere,
    &TorrentNoDeath,
    &FpsCap,
    &GameSpeed,
    &DisableLogos,
    &MuteMusic,
    &DisableAreaWelcomeMessage,
    &StutterFix,
    &MapInCombat,
    &TravelInDungeons,
    &DrawHitboxes,
    &ShowAllGraces,
    &ShowAllMaps,
    &DisableRoll,
    &DisableJump,
    &DisableBackstep,
];
