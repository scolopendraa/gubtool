use std::{thread, time::Duration};

use anyhow::{Result, bail};

use crate::{
    core::common::{rel_i32, write_to_slice},
    ds2::{
        mem::{ITEM_SPAWN_MUTEX, MASS_SPAWN_MUTEX, read, run_thread_release, write_bytes},
        offsets::{
            self,
            code_cave::{
                self,
                item_args_offsets::{self, *},
                item_struct_offsets,
            },
            functions, game_manager_imp,
        },
        resources::{
            asm,
            items::{
                Categories, Item,
                armor::ARMOR,
                arrows::ARROWS,
                consumables::CONSUMABLES,
                gestures::GESTURES,
                infusions::{INFUSION_IDS, INFUSIONS, Infusions},
                key_items::KEY_ITEMS,
                rings::RINGS,
                spells::SPELLS,
                upgrade_materials::UPGRADE_MATERIALS,
                weapons::WEAPONS,
            },
        },
        utils::{ScholarError, character_loaded_check, is_scholar},
    },
};


fn itemspawn(id: i32, stack_size: i32, durability: i32, quantity: i32, upgrade: i32, infusion: i32) -> Result<()> {
    let mut args: [u8; 35] = [0x0; 35];
    write_to_slice::<i32>(&mut args, item_args_offsets::CURRENT_QUANTITY, 0)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::STACK_COUNT, 0)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::MAX_QUANTITY, stack_size)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::ITEM_COUNT, 1)?;
    write_to_slice::<u8>(&mut args, item_args_offsets::ADJUST_QUANTITY_FLAG, stack_size > 1)?;
    write_to_slice::<u8>(&mut args, item_args_offsets::SHOULD_PROCESS_FLAG, 1)?;

    let item_struct = item_args_offsets::ITEM_STRUCT;
    write_to_slice::<i32>(&mut args, item_struct + item_struct_offsets::ITEM_ID, id)?;
    write_to_slice::<f32>(&mut args, item_struct + item_struct_offsets::DURABILITY, durability as f32)?;
    write_to_slice::<i16>(&mut args, item_struct + item_struct_offsets::QUANTITY, quantity)?;
    write_to_slice::<u8>(&mut args, item_struct + item_struct_offsets::UPGRADE, upgrade)?;
    write_to_slice::<u8>(&mut args, item_struct + item_struct_offsets::INFUSION, infusion)?;

    let _handle = ITEM_SPAWN_MUTEX.lock().unwrap();

    let args_location = code_cave::base() + code_cave::ITEM_ARGS;
    write_bytes(args_location, &args)?;

    let location = code_cave::base() + code_cave::ITEM_SPAWN_ASM;
    if read::<u8>(location)? == 0x0 {
        if is_scholar() {
            write_item_shellcode_scholar(args_location)?
        } else {
            write_item_shellcode_vanilla(args_location)?
        }
        run_thread_release(location)?
    }
    Ok(())
}

fn write_item_shellcode_scholar(args_loc: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::ITEM_SPAWN_ASM;
    let stack_space = code_cave::base() + code_cave::ITEM_SPAWN_STACK;
    let item_struct = args_loc + ITEM_STRUCT;
    use item_struct_offsets as off;

    let mut asm = asm::ITEM_SPAWN_SCHOLAR;
    write_to_slice::<u64>(&mut asm, 15, game_manager_imp::base())?;
    write_to_slice::<u64>(&mut asm, 87, functions::current_item_quantity_check())?;
    write_to_slice::<u64>(&mut asm, 175, functions::item_give())?;
    write_to_slice::<u64>(&mut asm, 215, functions::build_item_dialog())?;
    write_to_slice::<u64>(&mut asm, 242, functions::show_item_dialog())?;
    write_to_slice::<u64>(&mut asm, 262, read::<u64>(offsets::kernel32_sleep())?)?;
    write_to_slice::<i32>(&mut asm, 2, rel_i32(args_loc + SHOULD_PROCESS_FLAG, location + 7)?)?;
    write_to_slice::<i32>(&mut asm, 46, rel_i32(args_loc + SHOULD_PROCESS_FLAG, location + 51)?)?;
    write_to_slice::<i32>(&mut asm, 53, rel_i32(args_loc + ADJUST_QUANTITY_FLAG, location + 58)?)?;
    write_to_slice::<i32>(&mut asm, 67, rel_i32(args_loc + CURRENT_QUANTITY, location + 71)?)?;
    write_to_slice::<i32>(&mut asm, 74, rel_i32(args_loc + STACK_COUNT, location + 78)?)?;
    write_to_slice::<i32>(&mut asm, 81, rel_i32(item_struct + off::ITEM_ID, location + 85)?)?;
    write_to_slice::<i32>(&mut asm, 109, rel_i32(item_struct + off::QUANTITY, location + 113)?)?;
    write_to_slice::<i32>(&mut asm, 115, rel_i32(args_loc + CURRENT_QUANTITY, location + 119)?)?;
    write_to_slice::<i32>(&mut asm, 121, rel_i32(args_loc + MAX_QUANTITY, location + 125)?)?;
    write_to_slice::<i32>(&mut asm, 129, rel_i32(args_loc + MAX_QUANTITY, location + 133)?)?;
    write_to_slice::<i32>(&mut asm, 135, rel_i32(args_loc + CURRENT_QUANTITY, location + 139)?)?;
    write_to_slice::<i32>(&mut asm, 142, rel_i32(item_struct + off::QUANTITY, location + 146)?)?;
    write_to_slice::<i32>(&mut asm, 159, rel_i32(item_struct, location + 163)?)?;
    write_to_slice::<i32>(&mut asm, 166, rel_i32(args_loc + ITEM_COUNT, location + 170)?)?;
    write_to_slice::<i32>(&mut asm, 189, rel_i32(stack_space, location + 193)?)?;
    write_to_slice::<i32>(&mut asm, 196, rel_i32(item_struct, location + 200)?)?;
    write_to_slice::<i32>(&mut asm, 203, rel_i32(args_loc + ITEM_COUNT, location + 207)?)?;
    write_to_slice::<i32>(&mut asm, 236, rel_i32(stack_space, location + 240)?)?;
    write_to_slice::<i32>(&mut asm, 281, rel_i32(args_loc + SHOULD_EXIT_FLAG, location + 286)?)?;

    write_bytes(location, &asm)
}

fn write_item_shellcode_vanilla(args_loc: u64) -> Result<()> {
    let location = code_cave::base() + code_cave::ITEM_SPAWN_ASM;
    let stack_space = code_cave::base() + code_cave::ITEM_SPAWN_STACK;
    let item_struct = args_loc + ITEM_STRUCT;
    use item_struct_offsets as off;

    let mut asm = asm::ITEM_SPAWN_VANILLA;

    write_to_slice::<u32>(&mut asm, 2, args_loc + SHOULD_PROCESS_FLAG)?;
    write_to_slice::<u32>(&mut asm, 15, game_manager_imp::base())?;
    write_to_slice::<u32>(&mut asm, 32, args_loc + SHOULD_PROCESS_FLAG)?;
    write_to_slice::<u32>(&mut asm, 39, args_loc + ADJUST_QUANTITY_FLAG)?;
    write_to_slice::<u32>(&mut asm, 51, item_struct + off::ITEM_ID)?;
    write_to_slice::<u32>(&mut asm, 58, args_loc + STACK_COUNT)?;
    write_to_slice::<u32>(&mut asm, 65, args_loc + CURRENT_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 78, item_struct + off::QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 84, args_loc + CURRENT_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 90, args_loc + MAX_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 97, args_loc + MAX_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 103, args_loc + CURRENT_QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 109, item_struct + off::QUANTITY)?;
    write_to_slice::<u32>(&mut asm, 119, args_loc + ITEM_COUNT)?;
    write_to_slice::<u32>(&mut asm, 125, item_struct)?;
    write_to_slice::<u32>(&mut asm, 139, args_loc + ITEM_COUNT)?;
    write_to_slice::<u32>(&mut asm, 145, item_struct)?;
    write_to_slice::<u32>(&mut asm, 152, stack_space)?;
    write_to_slice::<u32>(&mut asm, 173, stack_space)?;
    write_to_slice::<u32>(&mut asm, 184, functions::sleep())?;
    write_to_slice::<u32>(&mut asm, 194, args_loc + SHOULD_EXIT_FLAG)?;
    write_to_slice::<i32>(&mut asm, 71, rel_i32(functions::current_item_quantity_check(), location + 75)?)?;
    write_to_slice::<i32>(&mut asm, 131, rel_i32(functions::item_give(), location + 135)?)?;
    write_to_slice::<i32>(&mut asm, 158, rel_i32(functions::build_item_dialog(), location + 162)?)?;
    write_to_slice::<i32>(&mut asm, 179, rel_i32(functions::show_item_dialog(), location + 183)?)?;

    write_bytes(location, &asm)
}

pub fn mass_spawn(category: Categories) -> Result<()> {
    let _handle = MASS_SPAWN_MUTEX.lock().unwrap();

    let items: &'static [Item] = match category {
            Categories::Armor => ARMOR,
            Categories::Arrows => ARROWS,
            Categories::Consumables => CONSUMABLES,
            Categories::Gestures => GESTURES,
            Categories::KeyItems => KEY_ITEMS,
            Categories::Rings => RINGS,
            Categories::Spells => SPELLS,
            Categories::UpgradeMaterials => UPGRADE_MATERIALS,
            Categories::Weapons => WEAPONS,
    };
    for item in items {
        if let Err(err) = item.spawn(1, 0, 0) &&
            !err.is::<ScholarError>() {
                return Err(err);
        }
        thread::sleep(Duration::from_millis(8));
    }
    Ok(())
}

impl Item {
    pub fn spawn(&self, quantity: i32, upgrade: i32, infusion: i32) -> Result<()> {
        character_loaded_check()?;
        if !is_scholar() && self.scholar_only {
            bail!(ScholarError)
        }
        itemspawn(
            self.id,
            self.stack_size,
            self.durability.unwrap_or(0),
            quantity,
            upgrade,
            infusion,
        )
    }
    pub fn available_infusions(&self) -> Vec<Infusions> {
        let mut infusions = Vec::new();
        if let Some(infusion_id) = self.infuse_id &&
        let Some(flags) = INFUSION_IDS.get(&infusion_id) {
            flags.iter().enumerate().for_each(|(idx, val)| {
                if *val == 1 {
                    infusions.push(INFUSIONS[idx]);
                }
            })
        }
        infusions
    }
}