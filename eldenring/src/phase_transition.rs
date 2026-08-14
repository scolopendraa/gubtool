use {
    crate::{
        chr_ins::ChrIns,
        resources::entity_ids,
        target::target,
        utils::{wait_for_cutscence_completion, wait_for_event},
    },
    anyhow::bail,
};

pub fn target_next_phase() -> anyhow::Result<()> {
    let mut target_guard = target();
    let chr = target_guard.chr_ins()?;
    match chr.entity_id()? {
        entity_ids::MARGIT_BOSS => {
            if !chr.has_speffect(16200)? {
                chr.set_hp_pct(65.0)?
            }
        }
        entity_ids::GODRICK => {
            if !chr.has_speffect(14750)? && chr.get_hp_pct()? > 60.1 {
                chr.set_hp_pct(60.0)?;
                chr.force_animation_playback(20010, false, false, false)?
            }
        }
        entity_ids::MORGOTT => {
            if !chr.has_speffect(16200)? {
                chr.set_hp_pct(60.0)?;
                chr.force_animation_playback(3024, false, false, false)?
            }
        }
        // entity_ids::STARSCOURGE_RADAHN => {
        // if true {
        // chr.set_hp_pct(50)?;
        // chr.force_animation_playback(3035, false, false, false)?
        // }
        // }
        entity_ids::DTS_BOSS => {
            if !chr.has_speffect(13708)? {
                chr.set_hp_pct(60.0)?;
                chr.force_animation_playback(3027, false, false, false)?
            }
        }
        entity_ids::CLERGYMAN => {
            chr.set_hp_pct(55.0)?;
            let mut maliketh_ins = ChrIns::from_entity_id(entity_ids::MALIKETH)?;
            maliketh_ins.set_hp_pct(55.0)?;
            wait_for_cutscence_completion()?;
            wait_for_event(13002802, true, 5)?;
            maliketh_ins.set_as_target(&mut target_guard)?
        }
        entity_ids::MOHG_LOB => {
            if !chr.has_speffect(10643)? {
                chr.set_hp_pct(50.0)?;
                chr.set_speffect(10641)?;
                chr.set_speffect(10642)?;
                chr.set_speffect(10643)?;
                chr.force_animation_playback(3004, false, false, false)?
            }
        }
        entity_ids::FORTISSAX => {
            if chr.get_hp_pct()? > 60.0 {
                chr.set_hp_pct(60.0)?;
            }
        }
        entity_ids::NOBLE_MANOR | entity_ids::NOBLE_DUO => {
            if chr.has_speffect(15500)? {
                chr.set_hp_pct(60.0)?;
                chr.force_animation_playback(3029, false, false, false)?
            }
        }
        entity_ids::FIRE_GIANT_P1 => {
            let mut p2_ins = ChrIns::from_entity_id(entity_ids::FIRE_GIANT_P2)?;
            let p1_max_hp = chr.get_max_hp()?;
            let p2_max_hp = p2_ins.get_max_hp()?;
            p2_ins.set_hp(p2_max_hp - p1_max_hp)?;
            chr.set_hp(0)?;
            wait_for_cutscence_completion()?;
            p2_ins.set_as_target(&mut target_guard)?
        }
        entity_ids::PLACIDUSAX => {
            if !chr.has_speffect(16890)? {
                chr.set_hp_pct(65.0)?;
                chr.set_speffect(16890)?;
                chr.force_animation_playback(3029, false, false, false)?;
            } else if !chr.has_speffect(16891)? {
                chr.set_hp_pct(45.0)?;
                chr.set_speffect(16891)?;
            } else if !chr.has_speffect(16892)? {
                chr.set_hp_pct(30.0)?;
                chr.set_speffect(16892)?;
            }
        }
        entity_ids::GOD_SERPENT => {
            if chr.get_hp_pct()? > 1.0 {
                chr.set_hp_pct(1.0)?;
                wait_for_cutscence_completion()?;
                ChrIns::from_entity_id(entity_ids::RYKARD)?.set_as_target(&mut target_guard)?;
            }
        }
        _ => bail!("Not implemented for current target"),
    }
    Ok(())
}
