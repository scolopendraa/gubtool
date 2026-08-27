use {
    crate::{
        enemy::{self, act_logger},
        mem::{read, write},
        offsets::{
            self,
            ChainReadExt,
            chr_ctrl::{boss_operator_offsets, chr_ai_manipulator_offsets},
            code_cave::CaveAddr,
        },
        speffect::{SpEffect, apply_speffect},
    },
    anyhow::ensure,
    gubtool_core::{address::Address, sys::sys_error::SysResult},
    std::collections::HashMap,
};

#[derive(Debug)]
pub struct ChrCtrl {
    pub resolved_pointers: HashMap<ResolvedChrPtr, u64>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ResolvedChrPtr {
    ChrCtrl,
    Stats,
    Params,
    BossOperator,
    ChrAiManipulator,
    ChrAi,
    SpEffectCtrl,
}

impl ChrCtrl {
    pub fn get_hp(&mut self) -> SysResult<i32> {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::HEALTH)
            .read::<i32>()
    }

    pub fn set_hp(&mut self, val: i32) -> SysResult {
        let max = self.max_hp()?;
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::HEALTH)
            .write::<i32>(val.min(max))
    }

    pub fn get_hp_pct(&mut self) -> SysResult<f32> {
        let current = self.get_hp()?;
        let max = self.max_hp()?;
        if max == 0 {
            return Ok(0.0);
        }
        Ok((current as f32 / max as f32) * 100.0)
    }

    pub fn set_hp_pct(&mut self, pct: f32) -> anyhow::Result<()> {
        let max = self.max_hp()?;
        ensure!(max != 0, "Could not set hp percentage: Tried to divide by zero");
        let val = (pct * max as f32) / 100.0;
        Ok(self.set_hp(val as i32)?)
    }

    pub fn get_min_hp(&mut self) -> SysResult<i32> {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::MIN_HEALTH)
            .read::<i32>()
    }

    pub fn set_min_hp(&mut self, val: i32) -> SysResult {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::MIN_HEALTH)
            .write::<i32>(val)
    }

    pub fn max_hp(&mut self) -> SysResult<i32> {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::MAX_HEALTH)
            .read::<i32>()
    }

    pub fn is_no_death(&mut self) -> bool {
        self.get_min_hp().map(|val| val == 1).unwrap_or_default()
    }

    pub fn set_no_death(&mut self, state: bool) -> SysResult {
        let val = if state { 1 } else { -99999 };
        self.set_min_hp(val)
    }

    pub fn coords(&mut self) -> SysResult<[f32; 3]> {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::COORDS)
            .read::<[f32; 3]>()
    }

    pub fn get_distance(&mut self, other: &mut ChrCtrl) -> SysResult<f32> {
        let self_pos = self.coords()?;
        let other_pos = other.coords()?;
        Ok(((other_pos[0] - self_pos[0]).powi(2)
            + (other_pos[1] - self_pos[1]).powi(2)
            + (other_pos[2] - self_pos[2]).powi(2))
        .sqrt())
    }

    pub fn chr_id(&mut self) -> SysResult<i32> {
        self.get_ptr(ResolvedChrPtr::Params)
            .add_offset(offsets::chr_ctrl::CHR_ID)
            .read::<i32>()
    }

    pub fn poise(&mut self) -> SysResult<f32> {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::POISE)
            .read::<f32>()
    }

    pub fn max_poise(&mut self) -> SysResult<f32> {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::MAX_POISE)
            .read::<f32>()
    }

    pub fn posture(&mut self) -> SysResult<f32> {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::POSTURE)
            .read::<f32>()
    }

    pub fn max_posture(&mut self) -> SysResult<f32> {
        self.get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::MAX_POSTURE)
            .read::<f32>()
    }

    pub fn rot_quaternion(&mut self) -> SysResult<[f32; 4]> {
        let [m00, m01, m02, _, m10, m11, m12, _, m20, m21, m22, _] = self
            .get_ptr(ResolvedChrPtr::ChrCtrl)
            .add_offset(offsets::chr_ctrl::ROTATION)
            .read::<[f32; 12]>()?;

        let matrix = glam::Mat3::from_cols(
            glam::Vec3::new(m00, m01, m02),
            glam::Vec3::new(m10, m11, m12),
            glam::Vec3::new(m20, m21, m22),
        );
        let q = glam::Quat::from_mat3(&matrix);

        Ok([q.x, q.y, q.z, q.w])
    }

    pub fn name_from_chr_id(&mut self) -> &'static str {
        crate::resources::chr_names::CHR_NAMES
            .get(&self.chr_id().unwrap_or_default())
            .map_or("", |v| *v)
    }

    pub fn last_act(&mut self) -> Option<i32> {
        let chr_ai = self.get_ptr(ResolvedChrPtr::ChrAi).ok()?;
        act_logger().get(chr_ai)
    }

    pub fn repeat_action(&mut self, act_id: i32) -> SysResult {
        let chr_ai = self.get_ptr(ResolvedChrPtr::ChrAi)?;
        write::<u64>(CaveAddr::ForceActChrAi, chr_ai)?;
        write::<i32>(CaveAddr::ForceActId, act_id)?;
        write::<u8>(CaveAddr::ForceActFlag, 0x1)
    }

    pub fn apply_speffect(&mut self, speffect: SpEffect) -> anyhow::Result<()> {
        let speffect_ctrl = self.get_ptr(ResolvedChrPtr::SpEffectCtrl)?;
        apply_speffect(speffect_ctrl, speffect)
    }

    pub fn repeat_last_action(&mut self, state: bool) -> SysResult {
        if state {
            if let Some(last_act) = self.last_act() {
                self.repeat_action(last_act)?;
            }
            Ok(())
        } else {
            write::<u8>(CaveAddr::ForceActFlag, 0x0)
        }
    }

    pub fn is_action_repeating(&mut self) -> SysResult<bool> {
        let chr_ai = self.get_ptr(ResolvedChrPtr::ChrAi)?;
        Ok(read::<u8>(CaveAddr::ForceActFlag).map(|val| val == 0x1)?
            && read::<u64>(CaveAddr::ForceActChrAi).map(|chr| chr == chr_ai)?)
    }

    pub fn is_ai_disabled(&mut self) -> SysResult<bool> {
        let chr_ai = self.get_ptr(ResolvedChrPtr::ChrAi)?;
        enemy::is_chr_ai_disabled(chr_ai)
    }

    pub fn set_disable_ai(&mut self, state: bool) -> SysResult {
        let chr_ai = self.get_ptr(ResolvedChrPtr::ChrAi)?;
        enemy::set_disable_chr_ai(chr_ai, state)
    }
}

impl ChrCtrl {
    pub fn new(pointer: impl Address) -> Self {
        let resolved_pointers = HashMap::new();
        let mut s = Self {
            resolved_pointers,
        };
        s.resolved_pointers
            .insert(ResolvedChrPtr::ChrCtrl, pointer.addr());
        s
    }

    pub fn is_valid_chr(&mut self) -> SysResult<bool> {
        if let Ok(ptr) = self.get_ptr(ResolvedChrPtr::ChrCtrl)
            && ptr == 0x0
        {
            return Ok(false);
        }
        let health = self.get_hp()?;
        let max_health = self.max_hp()?;
        Ok(health >= 0
            && max_health > 0
            && health < 10000000
            && max_health < 10000000
            && (health as f32) < (max_health as f32) * 1.5)
    }

    pub fn pointers(&self) -> Vec<(String, u64)> {
        self.resolved_pointers
            .iter()
            .map(|(name, addr)| (format!("{:?}", name), *addr))
            .collect()
    }

    pub fn get_ptr(&mut self, pointer: ResolvedChrPtr) -> SysResult<u64> {
        if let Some(&val) = self.resolved_pointers.get(&pointer) {
            return Ok(val);
        }

        let resolved_pointer = match pointer {
            ResolvedChrPtr::ChrCtrl => {
                unreachable!("ChrCtrl struct without base pointer")
            }
            ResolvedChrPtr::Stats => {
                self.get_ptr(ResolvedChrPtr::ChrCtrl)
                    .read_offset(offsets::chr_ctrl::STATS_PTR)
            }
            ResolvedChrPtr::Params => {
                self.get_ptr(ResolvedChrPtr::ChrCtrl)
                    .read_offset(offsets::chr_ctrl::PARAMS_PTR)
            }
            ResolvedChrPtr::SpEffectCtrl => {
                self.get_ptr(ResolvedChrPtr::ChrCtrl)
                    .read_offset(offsets::chr_ctrl::CHR_SPEFFECT_CTRL)
            }
            ResolvedChrPtr::BossOperator => {
                self.get_ptr(ResolvedChrPtr::ChrCtrl)
                    .read_offset(offsets::chr_ctrl::BOSS_OPERATOR)
            }
            ResolvedChrPtr::ChrAiManipulator => {
                self.get_ptr(ResolvedChrPtr::BossOperator)
                    .read_offset(boss_operator_offsets::CHR_AI_MANIPULATOR)
            }
            ResolvedChrPtr::ChrAi => {
                self.get_ptr(ResolvedChrPtr::ChrAiManipulator)
                    .read_offset(chr_ai_manipulator_offsets::CHR_AI)
            }
        }?;

        if resolved_pointer != 0x0 {
            self.resolved_pointers.insert(pointer, resolved_pointer);
        }
        Ok(resolved_pointer)
    }
}
