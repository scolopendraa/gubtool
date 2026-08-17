use {
    crate::{
        emevd,
        mem::*,
        offsets::{
            self,
            ChainReadExt,
            chr_ins::*,
            code_cave::CaveAddress,
            field_area,
            module_offsets::{BasePointer, Function},
            world_chr_man,
        },
        pointer_cache::ResolvedPtr,
        resources::{ASM, chr_names::CHR_NAMES},
        target::Target,
    },
    anyhow::{bail, ensure},
    gubtool_core::{
        address::Address,
        slice_ops::*,
        sys::{ipc::FfiValue, sys_error::ProcResult},
    },
    std::{collections::HashMap, sync::MutexGuard, time::Duration},
};

#[derive(Debug, Clone)]
pub struct ChrIns {
    pub resolved_pointers: HashMap<ResolvedChrPtr, u64>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ResolvedChrPtr {
    ChrIns,
    Modules,
    DataModule,
    SuperArmorModule,
    TimeActModule,
    BehaviourModule,
    PhysicsModule,
    RideModule,
    SpecialEffect,
    ComManipulator,
    AiThink,
    ChrCtrl,
    CtrlFlags,
}

impl ChrIns {
    pub fn get_current_hp(&mut self) -> ProcResult<i32> {
        self.get_ptr(ResolvedChrPtr::DataModule)
            .add_offset(data_offsets::HEALTH)
            .read::<i32>()
    }

    pub fn get_max_hp(&mut self) -> ProcResult<i32> {
        self.get_ptr(ResolvedChrPtr::DataModule)
            .add_offset(data_offsets::MAX_HEALTH)
            .read::<i32>()
    }

    pub fn set_hp(&mut self, val: i32) -> ProcResult {
        self.get_ptr(ResolvedChrPtr::DataModule)
            .add_offset(data_offsets::HEALTH)
            .write::<i32>(val)
    }

    pub fn get_hp_pct(&mut self) -> ProcResult<f32> {
        let current = self.get_current_hp()?;
        let max = self.get_max_hp()?;
        if max == 0 {
            return Ok(0.0);
        }
        Ok((current as f32 / max as f32) * 100.0)
    }

    pub fn set_hp_pct(&mut self, pct: f32) -> anyhow::Result<()> {
        let max = self.get_max_hp()?;
        ensure!(max != 0, "Could not set hp percentage: Tried to divide by zero");
        let val = (pct * max as f32) / 100.0;
        Ok(self.set_hp(val as i32)?)
    }

    pub fn set_no_death(&mut self, state: bool) -> ProcResult {
        self.get_ptr(ResolvedChrPtr::DataModule)
            .add_offset(offsets::chr_ins::data_flags())
            .set_bit(bit_flags::NO_DEATH, state)
    }

    pub fn is_no_death(&mut self) -> ProcResult<bool> {
        self.get_ptr(ResolvedChrPtr::DataModule)
            .add_offset(offsets::chr_ins::data_flags())
            .is_bit_set(bit_flags::NO_DEATH)
    }

    pub fn set_no_damage(&mut self, state: bool) -> ProcResult {
        self.get_ptr(ResolvedChrPtr::DataModule)
            .add_offset(offsets::chr_ins::data_flags())
            .set_bit(bit_flags::NO_DAMAGE, state)
    }

    pub fn is_no_damage(&mut self) -> ProcResult<bool> {
        self.get_ptr(ResolvedChrPtr::DataModule)
            .add_offset(offsets::chr_ins::data_flags())
            .is_bit_set(bit_flags::NO_DAMAGE)
    }

    pub fn get_max_poise(&mut self) -> ProcResult<f32> {
        self.get_ptr(ResolvedChrPtr::SuperArmorModule)
            .add_offset(super_armor_offsets::MAX_POISE)
            .read::<f32>()
    }

    pub fn get_current_poise(&mut self) -> ProcResult<f32> {
        self.get_ptr(ResolvedChrPtr::SuperArmorModule)
            .add_offset(super_armor_offsets::CURRENT_POISE)
            .read::<f32>()
    }

    pub fn get_poise_timer(&mut self) -> ProcResult<f32> {
        self.get_ptr(ResolvedChrPtr::SuperArmorModule)
            .add_offset(super_armor_offsets::POISE_TIMER)
            .read::<f32>()
    }

    pub fn get_current_animation(&mut self) -> ProcResult<i32> {
        self.get_ptr(ResolvedChrPtr::TimeActModule)
            .add_offset(time_act_offsets::ANIMATION_ID)
            .read::<i32>()
    }

    pub fn get_last_act(&mut self) -> ProcResult<u8> {
        self.get_ptr(ResolvedChrPtr::AiThink)
            .add_offset(ai_think_offsets::last_act())
            .read::<u8>()
    }

    pub fn set_repeat_last_act(&mut self, state: bool) -> ProcResult {
        let val = if state {
            self.get_last_act()?
        } else {
            0x0
        };
        self.get_ptr(ResolvedChrPtr::AiThink)
            .add_offset(ai_think_offsets::force_act())
            .write::<u8>(val)
    }

    pub fn is_repeat_act(&mut self) -> ProcResult<bool> {
        self.get_ptr(ResolvedChrPtr::AiThink)
            .add_offset(ai_think_offsets::force_act())
            .read::<u8>()
            .map(|val| val != 0x0)
    }

    pub fn repeat_act(&mut self, act: u8) -> ProcResult {
        self.get_ptr(ResolvedChrPtr::AiThink)
            .add_offset(ai_think_offsets::force_act())
            .write::<u8>(act)
    }

    pub fn force_act(&mut self, act: u8) -> ProcResult {
        self.repeat_act(act)?;
        while self.get_last_act()? != act {
            std::thread::sleep(Duration::from_millis(50));
        }
        self.set_repeat_last_act(false)
    }

    pub fn set_disable_ai(&mut self, state: bool) -> ProcResult {
        self.get_ptr(ResolvedChrPtr::CtrlFlags)
            .set_bit(bit_flags::DISABLE_AI, state)
    }

    pub fn is_disable_ai(&mut self) -> ProcResult<bool> {
        self.get_ptr(ResolvedChrPtr::CtrlFlags)
            .is_bit_set(bit_flags::DISABLE_AI)
    }

    pub fn get_animation_speed(&mut self) -> ProcResult<f32> {
        self.get_ptr(ResolvedChrPtr::BehaviourModule)
            .add_offset(behavior_offsets::ANIMATION_SPEED)
            .read::<f32>()
    }

    pub fn set_animation_speed(&mut self, val: f32) -> ProcResult {
        self.get_ptr(ResolvedChrPtr::BehaviourModule)
            .add_offset(behavior_offsets::ANIMATION_SPEED)
            .write::<f32>(val)
    }

    pub fn local_coords(&mut self) -> ProcResult<[f32; 3]> {
        self.get_ptr(ResolvedChrPtr::PhysicsModule)
            .add_offset(offsets::chr_ins::physics_offsets::COORDS)
            .read::<[f32; 3]>()
    }

    pub fn hurtbox_radius(&mut self) -> ProcResult<f32> {
        self.get_ptr(ResolvedChrPtr::PhysicsModule)
            .add_offset(offsets::chr_ins::physics_offsets::HURT_CAPSULE_RADIUS)
            .read::<f32>()
    }

    pub fn get_distance(&mut self, other: &mut ChrIns) -> ProcResult<f32> {
        let self_pos = self.local_coords()?;
        let other_pos = other.local_coords()?;
        let distance = ((other_pos[0] - self_pos[0]).powi(2)
            + (other_pos[1] - self_pos[1]).powi(2)
            + (other_pos[2] - self_pos[2]).powi(2))
        .sqrt();
        Ok(distance - self.hurtbox_radius()? - other.hurtbox_radius()?)
    }

    pub fn block_id(&mut self) -> ProcResult<u32> {
        self.get_ptr(ResolvedChrPtr::ChrIns)
            .add_offset(offsets::chr_ins::BLOCK_ID)
            .read::<u32>()
    }

    pub fn map_coords(&mut self) -> anyhow::Result<[f32; 3]> {
        let block_pos = world_block_info_from_block_id(self.block_id()?)
            .and_then(|addr| Ok(read::<[f32; 3]>(addr.saturating_add(0x70))?))?;
        let local_coords = self.local_coords()?;
        Ok([
            local_coords[0] - block_pos[0],
            local_coords[1] - block_pos[2],
            local_coords[1] - block_pos[2],
        ])
    }

    pub fn set_speffect(&mut self, speffect_id: u32) -> anyhow::Result<()> {
        let args = [
            FfiValue::pointer(self.get_ptr(ResolvedChrPtr::ChrIns)?),
            FfiValue::uint32(speffect_id),
        ];

        run_game_function(Function::SetSpeffect, &args)
    }

    pub fn remove_speffect(&mut self, speffect_id: u32) -> anyhow::Result<()> {
        let speffect_ptr = self.get_ptr(ResolvedChrPtr::SpecialEffect)?;
        let args = [
            FfiValue::pointer(speffect_ptr),
            FfiValue::uint32(speffect_id),
        ];

        run_game_function(Function::RemoveSpeffect, &args)
    }

    pub fn has_speffect(&mut self, speffect_id: u32) -> ProcResult<bool> {
        let mut current = self
            .get_ptr(ResolvedChrPtr::SpecialEffect)
            .read_offset(speffect_offsets::HEAD)?;
        while current != 0x0 {
            if read::<u32>(current.saturating_add(speffect_entry::ID))? == speffect_id {
                return Ok(true);
            }
            current = read::<u64>(current.saturating_add(speffect_entry::NEXT))?;
        }
        Ok(false)
    }

    pub fn reset_position(&mut self) -> anyhow::Result<()> {
        emevd::reset_character_position(self.entity_id()?)
    }

    pub fn force_animation_playback(
        &mut self,
        animation_id: u32,
        should_loop: bool,
        should_wait_for_completion: bool,
        ignore_wait_for_transition: bool,
    ) -> anyhow::Result<()> {
        emevd::force_animation_playback(
            self.entity_id()?,
            animation_id,
            should_loop,
            should_wait_for_completion,
            ignore_wait_for_transition,
        )
    }

    pub fn get_lua_timers(&mut self) -> ProcResult<[f32; 16]> {
        self.get_ptr(ResolvedChrPtr::AiThink)
            .add_offset(ai_think_offsets::LUA_TIMERS_ARRAY)
            .read::<[f32; 16]>()
    }

    pub fn set_as_target(mut self, target_guard: &mut MutexGuard<'static, Target>) -> ProcResult {
        write::<u64>(CaveAddress::SavedTargetPointer, self.get_ptr(ResolvedChrPtr::ChrIns)?)?;
        target_guard.set(self);
        Ok(())
    }

    pub fn chr_id(&mut self) -> ProcResult<i32> {
        self.get_ptr(ResolvedChrPtr::ChrIns)
            .add_offset(offsets::chr_ins::CHR_ID)
            .read::<i32>()
    }

    pub fn handle(&mut self) -> ProcResult<u64> {
        self.get_ptr(ResolvedChrPtr::ChrIns)
            .read_offset(offsets::chr_ins::HANDLE)
    }

    pub fn entity_id(&mut self) -> ProcResult<u32> {
        self.get_ptr(ResolvedChrPtr::ChrIns)
            .add_offset(offsets::chr_ins::entity_id())
            .read::<u32>()
    }

    pub fn npc_think_param_id(&mut self) -> ProcResult<i32> {
        self.get_ptr(ResolvedChrPtr::AiThink)
            .add_offset(ai_think_offsets::NPC_THINK_PARAM_ID)
            .read::<i32>()
    }

    pub fn name_from_chr_id(&mut self) -> &'static str {
        let chr_id = self.chr_id().unwrap_or_default();
        CHR_NAMES.get(&chr_id).map_or("", |v| *v)
    }
}

impl ChrIns {
    pub fn new(pointer: impl Address) -> Self {
        let resolved_pointers = HashMap::new();
        let mut s = Self {
            resolved_pointers,
        };
        s.resolved_pointers
            .insert(ResolvedChrPtr::ChrIns, pointer.addr());
        s
    }

    pub fn from_handle(handle: u64) -> Option<Self> {
        if handle == 0 {
            return None;
        }

        let pool_index = (handle >> 20) & 0xff;
        let slot_index = handle & 0xfffff;
        let pointer = ResolvedPtr::WorldChrMan
            .get()
            .read_offset(world_chr_man::chr_set_pool() + pool_index * 8)
            .read_offset(world_chr_man::chr_set_offsets::CHR_SET_ENTRIES)
            .read_offset(slot_index * 16)
            .unwrap_or_default();
        if pointer != 0 {
            Some(Self::new(pointer))
        } else {
            None
        }
    }

    pub fn from_entity_id(entity_id: u32) -> anyhow::Result<Self> {
        let mut fun = ASM.get_function("chr_ins_from_entity_id");
        let mut asm = fun.take_bytes();

        write_addr_to_slice(&mut asm, fun.reloc("world_chr_man"), BasePointer::WorldChrMan)?;
        write_to_slice::<u32>(&mut asm, fun.reloc("entity_id"), entity_id)?;
        write_addr_to_slice(&mut asm, fun.reloc("fn_chr_ins"), Function::GetChrInsByEntityId)?;
        write_addr_to_slice(&mut asm, fun.reloc("looked_up"), CaveAddress::LookedUpEntityId)?;

        run_custom_function(asm)?;
        let pointer = read::<u64>(CaveAddress::LookedUpEntityId)?;
        Ok(Self::new(pointer))
    }

    pub fn pointers(&self) -> Vec<(String, u64)> {
        self.resolved_pointers
            .iter()
            .map(|(name, addr)| (format!("{:?}", name), *addr))
            .collect()
    }

    pub fn get_ptr(&mut self, pointer: ResolvedChrPtr) -> ProcResult<u64> {
        if let Some(&val) = self.resolved_pointers.get(&pointer) {
            return Ok(val);
        }

        let resolved_pointer = match pointer {
            ResolvedChrPtr::ChrIns => {
                unreachable!("ChrIns struct without base pointer")
            }
            ResolvedChrPtr::Modules => {
                self.get_ptr(ResolvedChrPtr::ChrIns)
                    .read_offset(offsets::chr_ins::MODULES)
            }
            ResolvedChrPtr::DataModule => {
                self.get_ptr(ResolvedChrPtr::Modules)
                    .read_offset(offsets::chr_ins::CHR_DATA_MODULE)
            }
            ResolvedChrPtr::SuperArmorModule => {
                self.get_ptr(ResolvedChrPtr::Modules)
                    .read_offset(offsets::chr_ins::CHR_SUPER_ARMOR_MODULE)
            }
            ResolvedChrPtr::TimeActModule => {
                self.get_ptr(ResolvedChrPtr::Modules)
                    .read_offset(offsets::chr_ins::CHR_TIME_ACT_MODULE)
            }
            ResolvedChrPtr::BehaviourModule => {
                self.get_ptr(ResolvedChrPtr::Modules)
                    .read_offset(offsets::chr_ins::CHR_BEHAVIOR_MODULE)
            }
            ResolvedChrPtr::PhysicsModule => {
                self.get_ptr(ResolvedChrPtr::Modules)
                    .read_offset(offsets::chr_ins::CHR_PHYSICS_MODULE)
            }
            ResolvedChrPtr::RideModule => {
                self.get_ptr(ResolvedChrPtr::Modules)
                    .read_offset(offsets::chr_ins::CHR_RIDE_MODULE)
            }
            ResolvedChrPtr::SpecialEffect => {
                self.get_ptr(ResolvedChrPtr::ChrIns)
                    .read_offset(offsets::chr_ins::SPECIAL_EFFECT)
            }
            ResolvedChrPtr::ComManipulator => {
                self.get_ptr(ResolvedChrPtr::ChrIns)
                    .read_offset(offsets::chr_ins::com_manipulator())
            }
            ResolvedChrPtr::AiThink => {
                self.get_ptr(ResolvedChrPtr::ComManipulator)
                    .read_offset(0xc0)
            }
            ResolvedChrPtr::ChrCtrl => {
                self.get_ptr(ResolvedChrPtr::ChrIns)
                    .read_offset(offsets::chr_ins::CHR_CTRL)
            }
            ResolvedChrPtr::CtrlFlags => {
                self.get_ptr(ResolvedChrPtr::ChrCtrl)
                    .read_offset(0xc8)
                    .add_offset(0x24)
            }
        }?;

        if resolved_pointer != 0x0 {
            self.resolved_pointers.insert(pointer, resolved_pointer);
        }
        Ok(resolved_pointer)
    }
}

fn world_block_info_from_block_id(block_id: u32) -> anyhow::Result<u64> {
    let target_area = (block_id >> 24) & 0xff;
    let world_info_owner = read::<u64>(BasePointer::FieldArea)
        .and_then(|addr| read::<u64>(addr + field_area::WORLD_INFO_OWNER))?;
    let area_count =
        read::<i32>(world_info_owner + field_area::world_info_owner_offsets::AREA_COUNT)?;

    for i in 0..area_count as u64 {
        let area_ptr = read::<u64>(
            world_info_owner + field_area::world_info_owner_offsets::AREA_ARRAY_BASE + (i * 8),
        )?;
        let area_id = read::<u32>(area_ptr + 0xc)?;

        if area_id == target_area {
            let block_count = read::<i32>(area_ptr + 0x40)?;
            let blocks_ptr = read::<u64>(area_ptr + 0x48)?;

            for j in 0..block_count as u64 {
                let block_info_ptr = blocks_ptr + (j * 0xe0);
                let stored_block_id = read::<u32>(block_info_ptr + 0x8)?;

                if stored_block_id == block_id {
                    return Ok(block_info_ptr);
                }
            }
        }
    }
    bail!("Could not find world block info")
}
