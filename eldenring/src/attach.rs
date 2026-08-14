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
    apply_attach_entries(&ATTACH_ENTRIES)?;
    Ok(())
}

impl_attach_field_bool!(NoDeath, elden_ring, no_death, player::NoDeath);
impl_attach_field_bool!(NoDamage, elden_ring, no_damage, player::NoDamage);
impl_attach_field_bool!(InfinitePoise, elden_ring, infinite_poise, player::InfinitePoise);
impl_attach_field_bool!(OneShot, elden_ring, one_shot, player::OneShot);
impl_attach_field_bool!(RuneArc, elden_ring, rune_arc, player::RuneArc);
impl_attach_field_bool!(SetRfbsOnLoad, elden_ring, rfbs_on_load, player::SetRfbsOnLoad);
impl_attach_field_bool!(Hidden, elden_ring, hidden, player::Hidden);
impl_attach_field_bool!(Silent, elden_ring, silent, player::Silent);
impl_attach_field_bool!(InfiniteStamina, elden_ring, infinite_stamina, player::InfiniteStamina);
impl_attach_field_bool!(InfiniteFp, elden_ring, infinite_fp, player::InfiniteFp);
impl_attach_field_bool!(
    InfiniteConsumables,
    elden_ring,
    infinite_consumables,
    player::InfiniteConsumables
);
impl_attach_field_bool!(InfiniteArrows, elden_ring, infinite_arrows, player::InfiniteArrows);
impl_attach_field_bool!(TorrentAnywhere, elden_ring, torrent_anywhere, player::TorrentAnywhere);
impl_attach_field_bool!(TorrentNoDeath, elden_ring, torrent_no_death, player::TorrentNoDeath);
impl_attach_field_f32!(FpsCap, elden_ring, fps_cap, utility::FpsCap);
impl_attach_field_f32!(GameSpeed, elden_ring, game_speed, utility::GameSpeed);
impl_attach_field_bool!(DisableLogos, elden_ring, disable_logos, utility::DisableLogos);
impl_attach_field_bool!(MuteMusic, elden_ring, mute_music, utility::MuteMusic);
impl_attach_field_bool!(
    DisableAreaWelcomeMessage,
    elden_ring,
    disable_area_welcome_message,
    utility::DisableAreaWelcomeMessage
);
impl_attach_field_bool!(StutterFix, elden_ring, stutter_fix, utility::StutterFix);
impl_attach_field_bool!(MapInCombat, elden_ring, map_in_combat, utility::MapInCombat);
impl_attach_field_bool!(TravelInDungeons, elden_ring, travel_in_dungeon, utility::TravelInDungeons);
impl_attach_field_bool!(DrawHitboxes, elden_ring, draw_hitboxes, utility::DrawHitboxes);
impl_attach_field_bool!(ShowAllGraces, elden_ring, show_all_graces, utility::ShowAllGraces);
impl_attach_field_bool!(ShowAllMaps, elden_ring, show_all_maps, utility::ShowAllMaps);
impl_attach_field_bool!(DisableRoll, elden_ring, disable_roll, utility::DisableRoll);
impl_attach_field_bool!(DisableJump, elden_ring, disable_jump, utility::DisableJump);
impl_attach_field_bool!(DisableBackstep, elden_ring, disable_backstep, utility::DisableBackstep);

const ATTACH_ENTRIES: [&dyn AttachEntry; 28] = [
    &NoDeath,
    &NoDamage,
    &InfinitePoise,
    &OneShot,
    &RuneArc,
    &SetRfbsOnLoad,
    &Hidden,
    &Silent,
    &InfiniteStamina,
    &InfiniteFp,
    &InfiniteConsumables,
    &InfiniteArrows,
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
