use crate::{
    core::common::write_to_slice,
    er::{
        event,
        mem::*,
        offsets::{
            chr_ins::{self, *},
            code_cave, functions, world_chr_man,
        },
        resources::asm,
        target,
    },
};
use anyhow::{Result, anyhow, bail};
use phf::phf_map;

pub type ChrIns = Result<u64>;

pub trait ChrInsExt {
    fn get_current_hp(&self) -> Result<i32>;
    fn get_max_hp(&self) -> Result<i32>;
    fn set_hp(&self, val: i32) -> Result<()>;
    fn set_hp_pct(&self, val: i32) -> Result<()>;
    fn set_no_death(&self, val: bool) -> Result<()>;
    fn is_no_death(&self) -> Result<bool>;
    fn set_no_damage(&self, val: bool) -> Result<()>;
    fn is_no_damage(&self) -> Result<bool>;
    fn get_current_poise(&self) -> Result<f32>;
    fn get_max_poise(&self) -> Result<f32>;
    fn get_poise_timer(&self) -> Result<f32>;
    fn get_current_animation(&self) -> Result<i32>;
    fn get_last_act(&self) -> Result<u8>;
    fn set_repeat_act(&self, val: bool) -> Result<()>;
    fn is_repeat_act(&self) -> Result<bool>;
    fn force_act(&self, act: i32) -> Result<()>;
    fn set_disable_ai(&self, state: bool) -> Result<()>;
    fn is_disable_ai(&self) -> Result<bool>;
    fn get_animation_speed(&self) -> Result<f32>;
    fn set_animation_speed(&self, val: f32) -> Result<()>;
    fn local_coords(&self) -> Result<[f32; 3]>;
    fn map_coords(&self) -> Result<[f32; 3]>;
    fn hurtbox_radius(&self) -> Result<f32>;
    fn get_distance(&self, other: &ChrIns) -> Result<f32>;
    fn set_speffect(&self, speffect_id: i64) -> Result<()>;
    fn remove_speffect(&self, speffect_id: i64) -> Result<()>;
    //fn has_speffect(&self, speffect_id: i64) -> Result<()>;
    fn reset_position(&self) -> Result<()>;
    fn get_lua_timers(&self) -> Result<[f32; 16]>;

    fn chr_id(&self) -> Result<i32>;
    fn handle(&self) -> Result<u64>;
    fn entity_id(&self) -> Result<u32>;
    fn block_id(&self) -> Result<u32>;
    fn npc_think_param_id(&self) -> Result<i32>;

    fn chr_ins_pointer(&self) -> Result<u64>;
    fn modules(&self) -> Result<u64>;
    fn data_pointer(&self) -> Result<u64>;
    fn super_armor_pointer(&self) -> Result<u64>;
    fn time_act_pointer(&self) -> Result<u64>;
    fn physics_pointer(&self) -> Result<u64>;
    fn behaviour_pointer(&self) -> Result<u64>;
    fn ai_think_pointer(&self) -> Result<u64>;
    fn special_effect_pointer(&self) -> Result<u64>;
    fn ctrl_flags_pointer(&self) -> Result<u64>;
    fn ride_pointer(&self) -> Result<u64>;

    fn name_from_chr_id(&self) -> &'static str;
}

pub fn chr_ins_from_entity_id(entity_id: u32) -> ChrIns {
    let location = code_cave::base() + code_cave::CHR_INS_FROM_ENTITY_ID_ASM;
    let looked_up_chr_ins = code_cave::base() + code_cave::LOOKED_UP_CHR_INS;
    let world_chr_man = read::<u64>(world_chr_man::base())?;

    let mut asm = asm::CHR_INS_FROM_ENTITY_ID;
    write_to_slice::<u64>(&mut asm, 2, world_chr_man)?;
    write_to_slice::<u32>(&mut asm, 17, entity_id)?;
    write_to_slice::<u64>(&mut asm, 23, functions::get_chr_ins_by_entity_id())?;
    write_to_slice::<u64>(&mut asm, 39, looked_up_chr_ins)?;
    let asm = append_flag_setter(location, &asm)?;

    write_bytes(location, &asm)?;
    run_thread(location)?;
    read::<u64>(looked_up_chr_ins)
}

pub fn chr_ins_from_handle(handle: u64) -> ChrIns {
    let pool_index = (handle >> 20) & 0xFF;
    let slot_index = handle & 0xFFFFF;
    read::<u64>(world_chr_man::base())
        .and_then(|addr| read::<u64>(addr + world_chr_man::chr_set_pool() + pool_index * 8))
        .and_then(|addr| read::<u64>(addr + world_chr_man::chr_set_offsets::CHR_SET_ENTRIES))
        .and_then(|addr| read::<u64>(addr + slot_index * 16))
}

impl ChrInsExt for ChrIns {
    fn get_current_hp(&self) -> Result<i32> {
        read::<i32>(self.data_pointer()? + data_offsets::HEALTH)
    }

    fn get_max_hp(&self) -> Result<i32> {
        read::<i32>(self.data_pointer()? + data_offsets::MAX_HEALTH)
    }

    fn set_hp(&self, val: i32) -> Result<()> {
        write::<i32>(self.data_pointer()? + data_offsets::HEALTH, val)
    }

    fn set_hp_pct(&self, pct: i32) -> Result<()> {
        let max_hp = self.get_max_hp()?;
        if max_hp == 0 {
            bail!("Could not set hp percentage: Tried to divide by zero")
        }
        let val = (pct * max_hp) / 100;
        write::<i32>(self.data_pointer()? + data_offsets::HEALTH, val)
    }

    fn set_no_death(&self, state: bool) -> Result<()> {
        set_bit(self.data_pointer()? + data_flags(), bit_flags::NO_DEATH, state)
    }

    fn is_no_death(&self) -> Result<bool> {
        is_bit_set(self.data_pointer()? + data_flags(), bit_flags::NO_DEATH)
    }

    fn set_no_damage(&self, state: bool) -> Result<()> {
        set_bit(self.data_pointer()? + data_flags(), bit_flags::NO_DAMAGE, state)
    }

    fn is_no_damage(&self) -> Result<bool> {
        is_bit_set(self.data_pointer()? + data_flags(), bit_flags::NO_DAMAGE)
    }

    fn get_max_poise(&self) -> Result<f32> {
        read::<f32>(self.super_armor_pointer()? + super_armor_offsets::MAX_POISE)
    }

    fn get_current_poise(&self) -> Result<f32> {
        read::<f32>(self.super_armor_pointer()? + super_armor_offsets::CURRENT_POISE)
    }

    fn get_poise_timer(&self) -> Result<f32> {
        read::<f32>(self.super_armor_pointer()? + super_armor_offsets::POISE_TIMER)
    }

    fn get_current_animation(&self) -> Result<i32> {
        read::<i32>(self.time_act_pointer()? + time_act_offsets::ANIMATION_ID)
    }

    fn get_last_act(&self) -> Result<u8> {
        read::<u8>(self.ai_think_pointer()? + ai_think_offsets::last_act())
    }

    fn set_repeat_act(&self, state: bool) -> Result<()> {
        write::<u8>(self.ai_think_pointer()? + ai_think_offsets::force_act(), state as u8)
    }

    fn is_repeat_act(&self) -> Result<bool> {
        read::<u8>(self.ai_think_pointer()? + ai_think_offsets::force_act())
            .map(|val| val != 0x0)
    }

    fn force_act(&self, act: i32) -> Result<()> {
        write::<i32>(self.ai_think_pointer()? + ai_think_offsets::force_act(), act)
    }

    fn set_disable_ai(&self, state: bool) -> Result<()> {
        set_bit(self.ctrl_flags_pointer()?, bit_flags::DISABLE_AI, state)
    }

    fn is_disable_ai(&self) -> Result<bool> {
        is_bit_set(self.ctrl_flags_pointer()?, bit_flags::DISABLE_AI)
    }

    fn get_animation_speed(&self) -> Result<f32> {
        read::<f32>(self.behaviour_pointer()? + behavior_offsets::ANIMATION_SPEED)
    }

    fn set_animation_speed(&self, val: f32) -> Result<()> {
        write::<f32>(self.behaviour_pointer()? + behavior_offsets::ANIMATION_SPEED, val)
    }

    fn local_coords(&self) -> Result<[f32; 3]> {
        read::<[f32; 3]>(self.physics_pointer()? + chr_ins::physics_offsets::COORDS)
    }

    fn hurtbox_radius(&self) -> Result<f32> {
        read::<f32>(self.physics_pointer()? + chr_ins::physics_offsets::HURT_CAPSULE_RADIUS)
    }

    fn get_distance(&self, other: &ChrIns) -> Result<f32> {
        let self_pos = self.local_coords()?;
        let other_pos = other.local_coords()?;
        let distance = (
            (other_pos[0] - self_pos[0]).powi(2) +
            (other_pos[1] - self_pos[1]).powi(2) +
            (other_pos[2] - self_pos[2]).powi(2))
            .sqrt();
        Ok(distance - self.hurtbox_radius()? - other.hurtbox_radius()?)
    }

    fn block_id(&self) -> Result<u32> {
        read::<u32>(self.chr_ins_pointer()? + chr_ins::BLOCK_ID)
    }

    fn map_coords(&self) -> Result<[f32; 3]> {
        let block_pos = target::world_block_info_from_block_id(self.block_id()?)
            .and_then(|addr| read::<[f32; 3]>(addr + 0x70))?;
        let local_coords = self.local_coords()?;
        Ok([
            local_coords[0] - block_pos[0],
            local_coords[1] - block_pos[2],
            local_coords[1] - block_pos[2],
        ])
    }

    fn set_speffect(&self, speffect_id: i64) -> Result<()> {
        let location = code_cave::base() + code_cave::SET_SPEFFECT_ASM;

        let mut asm = asm::SET_SPEFFECT;
        write_to_slice::<u64>(&mut asm, 2, self.chr_ins_pointer()?)?;
        write_to_slice::<i64>(&mut asm, 12, speffect_id)?;
        write_to_slice::<u64>(&mut asm, 22, functions::set_speffect())?;
        let asm = append_flag_setter(location, &asm)?;

        write_bytes(location, &asm)?;
        run_thread(location)
    }

    fn remove_speffect(&self, speffect_id: i64) -> Result<()> {
        let location = code_cave::base() + code_cave::REMOVE_SPEFFECT_ASM;

        let mut asm = asm::REMOVE_SPEFFECT;
        write_to_slice::<u64>(&mut asm, 2, self.special_effect_pointer()?)?;
        write_to_slice::<i64>(&mut asm, 12, speffect_id)?;
        write_to_slice::<u64>(&mut asm, 22, functions::remove_speffect())?;
        let asm = append_flag_setter(location, &asm)?;

        write_bytes(location, &asm)?;
        run_thread(location)
    }

    fn reset_position(&self) -> Result<()> {
        event::reset_character_position(self.entity_id()?)
    }

    fn get_lua_timers(&self) -> Result<[f32; 16]> {
        read::<[f32; 16]>(self.ai_think_pointer()? + ai_think_offsets::LUA_TIMERS_ARRAY)
    }

    fn chr_id(&self) -> Result<i32> {
        read::<i32>(self.chr_ins_pointer()? + chr_ins::CHR_ID)
    }

    fn handle(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()? + chr_ins::HANDLE)
    }

    fn entity_id(&self) -> Result<u32> {
        read::<u32>(self.chr_ins_pointer()? + chr_ins::entity_id())
    }

    fn npc_think_param_id(&self) -> Result<i32> {
        read::<i32>(self.ai_think_pointer()? + ai_think_offsets::NPC_THINK_PARAM_ID)
    }

    fn chr_ins_pointer(&self) -> Result<u64> {
        Ok(*self.as_ref().map_err(|e| anyhow!("{e}"))?)
    }

    fn modules(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::MODULES))
    }

    fn data_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_DATA_MODULE))
    }

    fn super_armor_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_SUPER_ARMOR_MODULE))
    }

    fn time_act_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_TIME_ACT_MODULE))
    }

    fn behaviour_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_BEHAVIOR_MODULE))
    }

    fn physics_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_PHYSICS_MODULE))
    }

    fn ai_think_pointer(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::manipulator()))
            .and_then(|addr| read::<u64>(addr + 0xC0))
    }

    fn special_effect_pointer(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::SPECIAL_EFFECT))
    }

    fn ctrl_flags_pointer(&self) -> Result<u64> {
        read::<u64>(self.chr_ins_pointer()?.saturating_add(chr_ins::CHR_CTRL))
            .and_then(|addr| read::<u64>(addr + 0xC8))
            .map(|addr| addr + 0x24)
    }

    fn ride_pointer(&self) -> Result<u64> {
        read::<u64>(self.modules()?.saturating_add(chr_ins::CHR_RIDE_MODULE))
    }

    fn name_from_chr_id(&self) -> &'static str {
        CHR_NAMES
            .get(&self.chr_id().unwrap_or_default())
            .map_or("", |v| *v)
    }
}

static CHR_NAMES: phf::Map<i32, &'static str> = phf_map! {
    2010 => "Blaidd",
    2030 => "Rennala",
    2031 => "Rennala",
    2040 => "Juvenile Scholar",
    2041 => "Larva of Rot",
    2050 => "Ranni the Witch",
    2051 => "Ranni the Witch",
    2060 => "The Two Fingers",
    2070 => "The Three Fingers",
    2100 => "Black Knife Assassin",
    2110 => "Beast Clergyman",
    2120 => "Malenia, Blade of Miquella",
    2130 => "Morgott, the Omen King",
    2131 => "Morgott (Dead)",
    2140 => "Omen",
    2150 => "Lightning Ball",
    2160 => "Finger Reader Crone",
    2170 => "Finger Reader Crone",
    2180 => "Melina",
    2190 => "Radagon of the Golden Order",
    2191 => "Radagon (Crucified)",
    2200 => "Elden Beast",
    2270 => "Giant Crab",
    2271 => "Crab",
    2272 => "Giant Black Crab",
    2273 => "Black Crab",
    2274 => "Giant Albinauric Crab",
    2275 => "Albinauric Crab",
    2276 => "Giant Death Crab",
    2277 => "Death Crab",
    2500 => "Crucible Knight",
    3000 => "Exile Soldier",
    3010 => "Banished Knight",
    3020 => "Exile Soldier (Large)",
    3050 => "Commander Niall/O'Neill",
    3060 => "Giant Skeleton",
    3061 => "Giant Beast Skeleton",
    3070 => "Dominula Celebrant",
    3080 => "Imp",
    3100 => "Elemer of the Briar",
    3150 => "Night's Cavalry",
    3160 => "Funeral Steed",
    3170 => "Albinauric Archer",
    3171 => "Phillia Towering Little Sister",
    3180 => "Albinauric Archer's Wolf",
    3181 => "Red Wolf of Radagon",
    3200 => "Nomadic Merchant",
    3201 => "Frenzied Nomad",
    3202 => "Nomadic Merchant",
    3210 => "Nomadic Merchant's Donkey",
    3250 => "Draconic Tree Sentinel",
    3251 => "Tree Sentinel",
    3252 => "Loretta",
    3300 => "Nox Swordstress/Monk/Nightmaiden",
    3320 => "Silver Tear",
    3330 => "Giant Silver Tear",
    3350 => "Crystalian",
    3360 => "Ancestral Follower",
    3361 => "Putrid Ancestral Follower",
    3370 => "Ancestral Follower Shaman",
    3371 => "Putrid Ancestral Follower Shaman",
    3400 => "Grave Warden Duelist",
    3450 => "Misbegotten",
    3451 => "Scaly Misbegotten",
    3460 => "Leonine Misbegotten",
    3470 => "Albinauric",
    3471 => "Large Albinauric",
    3500 => "Skeletal Militiaman/Bandit/Knight/Grave Warden",
    3510 => "Catacomb Skeleton",
    3550 => "Sanguine Noble",
    3560 => "Godskin Apostle",
    3570 => "Godskin Noble",
    3600 => "Alabaster Lord",
    3610 => "Oracle Envoy",
    3620 => "Oracle Envoy (Large)",
    3630 => "Oracle Envoy (Giant)",
    3650 => "Erdtree Guardian",
    3660 => "Commoner",
    3661 => "Putrid Corpse",
    3662 => "Putrid Corpse (Large)",
    3664 => "Cemetery Shade",
    3665 => "Gostoc",
    3666 => "The Noble Goldmask",
    3670 => "Elder Albinauric",
    3700 => "Depraved Perfumer",
    3701 => "Perfumer",
    3702 => "Glintstone Sorcerer",
    3703 => "Page",
    3704 => "Battlemage",
    3710 => "Azur",
    3720 => "Lusat",
    3730 => "Graven School",
    3750 => "Clayman",
    3800 => "Cleanrot Knight",
    3810 => "Kindred of Rot",
    3850 => "Marionette Soldier",
    3860 => "Avionette Soldier",
    3900 => "Fire Monk",
    3901 => "Blackflame Monk",
    3910 => "Fire Prelate",
    3950 => "Man-Serpent",
    3970 => "Azula Beastman",
    4000 => "Revenant Follower",
    4020 => "Royal Revenant",
    4040 => "Slug",
    4050 => "Kaiden Sellsword",
    4060 => "Kaiden Sellsword's Horse",
    4070 => "Wolf",
    4071 => "White Wolf",
    4080 => "Rat",
    4090 => "Giant Rat",
    4100 => "Demi-Human",
    4101 => "Demi-Human (Large)",
    4110 => "Demi-Human Shaman",
    4120 => "Demi-Human Chief",
    4130 => "Demi-Human Queen",
    4140 => "Snail",
    4150 => "Basilisk",
    4160 => "Stray (Large)",
    4161 => "Stray",
    4162 => "Farum Azula Stray (Large)",
    4163 => "Farum Azula Stray",
    4164 => "Bloodbane Stray (Large)",
    4165 => "Bloodbane Stray",
    4166 => "Rotten Stray (Large)",
    4167 => "Rotten Stray",
    4170 => "Putrid Flesh",
    4171 => "Putrid Flesh (Large)",
    4180 => "Spirit Jellyfish",
    4190 => "Large Scarab",
    4191 => "Scarab",
    4192 => "Giant Scarab",
    4200 => "Man-Bat",
    4201 => "Operatic Bat",
    4210 => "Warhawk",
    4220 => "Land Octopus (Large)",
    4230 => "Land Octopus (Small)",
    4240 => "Fingercreeper",
    4241 => "Fingercreeper (Large)",
    4250 => "Fingercreeper (Small)",
    4260 => "Erdtree Burial Watchdog",
    4270 => "Elder Lion",
    4280 => "Giant Ant",
    4281 => "Giant Ant (Skull Plate)",
    4290 => "Bloodhound Knight",
    4300 => "Wandering Noble",
    4310 => "Lordsworn Soldier",
    4311 => "Godrick Soldier",
    4312 => "Raya Lucaria Soldier",
    4313 => "Leyndell Soldier",
    4314 => "Radahn Soldier",
    4315 => "Mausoleum Soldier",
    4316 => "Haligtree Soldier",
    4321 => "Vulgar Militia",
    4340 => "Mad Pumpkin Head",
    4341 => "Mad Pumpkin Head (Thin)",
    4350 => "Lordsworn Knight",
    4351 => "Godrick Knight",
    4352 => "Cuckoo Knight",
    4353 => "Leyndell Knight",
    4354 => "Redmane Knight",
    4355 => "Mausoleum Knight",
    4356 => "Haligtree Knight",
    4360 => "Lordsworn Knight's Horse",
    4361 => "Godrick Knight's Horse",
    4362 => "Cuckoo Knight's Horse",
    4363 => "Leyndell Knight's Horse",
    4364 => "Redmane Knight's Horse",
    4365 => "Mausoleum Knight's Horse",
    4370 => "Foot Soldier",
    4371 => "Godrick Foot Soldier",
    4372 => "Raya Lucaria Foot Soldier",
    4373 => "Leyndell Foot Soldier",
    4374 => "Radahn Foot Soldier",
    4375 => "Mausoleum Foot Soldier",
    4376 => "Haligtree Foot Soldier",
    4377 => "Highwayman",
    4380 => "Starcaller",
    4381 => "Guilty",
    4382 => "Stonedigger",
    4383 => "Glintstone Digger",
    4384 => "Glintstone Digger (Small Sack)",
    4385 => "Disciple of Rot",
    4420 => "Giant Crayfish",
    4421 => "Giant Crayfish",
    4430 => "Watcher Stones",
    4440 => "Land Squirt",
    4441 => "Giant Land Squirt",
    4442 => "Giant Rotten Land Squirt",
    4450 => "Walking Mausoleum",
    4460 => "Flame Chariot",
    4470 => "Abductor Virgin",
    4480 => "Miranda Blossom",
    4481 => "Miranda Sprout",
    4482 => "Rotten Miranda Blossom",
    4483 => "Rotten Miranda Sprout",
    4490 => "Living Jar Warrior",
    4491 => "Living Jar",
    4492 => "Greatjar",
    4500 => "Flying Dragon",
    4501 => "Decaying Ekzykes",
    4502 => "Glintstone Dragon",
    4503 => "Borealis the Freezing Fog",
    4504 => "Elder Dragon Greyoll",
    4505 => "Flying Dragon (Small)",
    4510 => "Ancient Dragon",
    4511 => "Lichdragon Fortissax",
    4520 => "Dragonlord Placidusax",
    4550 => "Giant Dog",
    4560 => "Giant Crow",
    4561 => "Giant Crow (Rotten)",
    4570 => "Wormface",
    4580 => "Large Wormface",
    4600 => "Troll",
    4601 => "Troll Knight (Carian Blade)",
    4602 => "Snowfield Troll",
    4603 => "Stonedigger Troll",
    4604 => "War Counselor Iji",
    4620 => "Astel, Naturalborn of the Void",
    4630 => "Runebear",
    4640 => "Ulcerated Tree Spirit",
    4650 => "Dragonkin Soldier",
    4660 => "Guardian Golem",
    4670 => "Ancestor Spirit",
    4680 => "Falling-star Beast",
    4690 => "Grafted Scion",
    4710 => "God-Devouring Serpent / Rykard Lord of Blasphemy",
    4711 => "Rykard's Corpse",
    4720 => "Godfrey, First Elden Lord",
    4721 => "Hoarah Loux, Warrior",
    4730 => "Starscourge Radahn",
    4750 => "Godrick the Grafted",
    4751 => "Godrick's Corpse",
    4760 => "Fire Giant",
    4770 => "Valiant Gargoyle",
    4800 => "Mohg, Lord of Blood",
    4801 => "Mohg, Lord of Blood (Unknown)",
    4810 => "Erdtree Avatar",
    4811 => "Putrid Avatar",
    4820 => "Omenkiller",
    4910 => "Magma Wyrm",
    4911 => "Great Wyrm Theodorix",
    4950 => "Tibia Mariner",
    4960 => "Giant Skeleton Torso",
    4980 => "Death Rite Bird",
    5000 => "Commander Gaius",
    5010 => "Golden Hippopotamus",
    5011 => "Golden Hippopotamus (Golden Wings)",
    5020 => "Putrescent Knight",
    5030 => "Romina, Saint of the Bud",
    5040 => "Curseblade Labirith",
    5050 => "Midra, Lord of Frenzied Flame",
    5051 => "Midra, Lord of Frenzied Flame",
    5060 => "Lamprey",
    5061 => "Lamprey (Large)",
    5070 => "Death Knight",
    5080 => "Bloodfiend",
    5081 => "Chief Bloodfiend",
    5090 => "Gravebird",
    5110 => "Ghostly Worm (TEMP NAME)",
    5120 => "Bayle the Dread",
    5130 => "Messmer the Impaler",
    5131 => "Messmer the Impaler",
    5132 => "Messmer the Impaler",
    5140 => "Messmer: Base Serpent",
    5141 => "Messmer: Base Serpent",
    5160 => "Fire Knight",
    5170 => "Furnace Golem",
    5180 => "Dummy Entity (TEMP NAME)",
    5181 => "Dummy Entity (TEMP NAME)",
    5190 => "Spider Scorpion",
    5192 => "Spider Scorpion",
    5193 => "Spider Scorpion (Small)",
    5194 => "Spider Scorpion (Small)",
    5200 => "Metyr, Mother of Fingers",
    5210 => "Divine Beast Dancing Lion",
    5220 => "Promised Consort Radahn",
    5221 => "Miquella's Arms",
    5230 => "Scadutree Avatar",
    5240 => "Commoner (Pot)",
    5241 => "Commoner",
    5250 => "Horned Warrior",
    5251 => "Horned Shaman",
    5260 => "Smith Golem",
    5261 => "Smith Golem",
    5270 => "Jar Innards",
    5271 => "Jar Innards",
    5280 => "Houzuki (TEMP NAME)",
    5300 => "Rellana Twin Moon Knight",
    5311 => "Inquisitor (Candles)",
    5312 => "Inquisitor (Staff)",
    5320 => "Jori Elder Inquisitor",
    5330 => "St. Trina",
    5340 => "Dummy Entity (TEMP NAME)",
    5350 => "Basilisk Eyes",
    5360 => "Giant Beast Skeleton",
    5370 => "Ancient Dragon Senessax",
    5380 => "Miranda Blossom",
    5381 => "Miranda Sprout",
    5390 => "Troll",
    5391 => "Troll Knight",
    5401 => "Eagle",
    5410 => "Deer",
    5421 => "Red Bear",
    5430 => "Owl",
    5440 => "Boar",
    5450 => "Ram",
    5460 => "Murre",
    5470 => "Dragonfly",
    5471 => "Yellow Dragonfly",
    5472 => "Great Dragonfly",
    5480 => "Turtle",
    5490 => "Springhare",
    5500 => "Living Magma",
    5511 => "Shade",
    5512 => "Shade",
    5513 => "Cemetary Shade",
    5520 => "Stray",
    5521 => "Stray",
    5522 => "Stray",
    5523 => "Stray",
    5524 => "Braided Stray",
    5525 => "Braided Stray",
    5526 => "Bloodbane Stray",
    5527 => "Bloodbane Stray",
    5530 => "Man-Bat",
    5540 => "Wolf",
    5541 => "Wolf",
    5550 => "Fingercreeper",
    5551 => "Fingercreeper (Large)",
    5560 => "Fingercreeper (Small)",
    5570 => "Marionette Soldier",
    5580 => "Flying Dragon",
    5590 => "Putrid Flesh",
    5591 => "Putrid Flesh (Large)",
    5600 => "Catacomb Skeleton",
    5620 => "Tibia Mariner",
    5630 => "Giant Crayfish",
    5640 => "Giant Crab",
    5641 => "Crab",
    5651 => "Messmer Foot Soldier",
    5661 => "Shadow Militia",
    5680 => "Wheeled Ballista",
    5690 => "Dragon-faced Flamethrower",
    5700 => "Demi-Human",
    5701 => "Demi-Human (Large)",
    5710 => "Demi-Human Shaman",
    5720 => "Demi-Human Chief",
    5730 => "Demi-Human Queen",
    5740 => "Kindred of Rot",
    5750 => "Living Jar Warrior",
    5751 => "Living Jar",
    5760 => "Rat",
    5761 => "Giant Rat",
    5780 => "Runebear",
    5790 => "Guardian Golem",
    5800 => "Crucible Knight Devonia",
    5810 => "Demi-Human Swordmaster Onze",
    5820 => "Great Red Bear",
    5830 => "Messmer Soldier",
    5840 => "Black Knight",
    5850 => "Giant Ram",
    5860 => "Ghostflame Dragon",
    5870 => "Imp (Lion Head)",
    5871 => "Imp (Dragon Head)",
    5872 => "Imp (Dragon Head)",
    5880 => "Putrescence Sorcerer",
    5890 => "Black Knight Horse",
    5900 => "Man-Fly",
    5910 => "Hornsent Grandam",
    5920 => "Magma Wyrm",
    5930 => "Giant Skeleton",
    5931 => "Giant Beast Skeleton",
    5940 => "Watcher Stones",
    5950 => "Leonine Misbegotten",
    5960 => "Ulcerated Tree Spirit",
    5970 => "Abductor Virgin",
    5980 => "Omenkiller",
    5990 => "Basilisk",
    6001 => "Eagle",
    6010 => "Deer",
    6031 => "Bear",
    6040 => "Owl",
    6050 => "Boar",
    6060 => "Goat",
    6070 => "Murre",
    6072 => "Carrier Pigeon",
    6080 => "Dragonfly",
    6081 => "Yellow Dragonfly",
    6082 => "Great Dragonfly",
    6090 => "Turtle",
    6091 => "Miriel Pastor of Vows",
    6100 => "Springhare",
    6201 => "Scarab",
    6210 => "Albinauric Archer",
    6220 => "Wolf",
    6231 => "Perfumer",
    6232 => "Glintstone Sorcerer",
    6233 => "Page",
    6240 => "Wisp",
    6251 => "Tree Sentinel",
    6260 => "Death Rite Bird",
    6270 => "Royal Revenant",
    6290 => "Misbegotten",
    6291 => "Scaly Misbegotten",
    6300 => "Snail",
    6310 => "Fallingstar Beast",
    6320 => "Wandering Noble",
    7000 => "Fallen Hawks Soldier",
    7100 => "Ancient Hero",
    8000 => "Torrent",
    8100 => "Ballista",
    8101 => "Wheeled Ballista",
    8110 => "Dragon-faced Flamethrower",
    8120 => "Merciless Chariot",
    8900 => "Malenia, Blade of Miquella",
    8901 => "Beast Clergyman",
    8902 => "Miquella",
    8903 => "Rennala, Queen of the Full Moon",
    8904 => "Morgott",
    8905 => "Mohg, Lord of Blood",
    8906 => "Melina",
    8907 => "Ranni Doll",
    9001 => "FLVER Test",
};
