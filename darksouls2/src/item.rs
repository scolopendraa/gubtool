use {
    crate::{
        mem::*,
        offsets::{
            code_cave::{
                CaveAddress,
                item_args_offsets::{self, *},
                item_struct_offsets,
            },
            module_offsets::{BasePointer, Function},
        },
        resources::{
            asm_function,
            items::{
                Categories,
                Item,
                armor::ARMOR,
                arrows::ARROWS,
                consumables::CONSUMABLES,
                gestures::GESTURES,
                infusions::{INFUSION_IDS, INFUSIONS, Infusion},
                key_items::KEY_ITEMS,
                rings::RINGS,
                spells::SPELLS,
                upgrade_materials::UPGRADE_MATERIALS,
                weapons::WEAPONS,
            },
        },
        utils::{ScholarError, player_loaded_check, scholar_check},
    },
    gubtool_core::{address::Address, slice_ops::*},
    std::{thread, time::Duration},
};

#[derive(Clone)]
pub struct ItemSpawnRequest {
    pub item:     Item,
    pub quantity: u32,
    pub upgrade:  u32,
    pub infusion: Infusion,
}

impl ItemSpawnRequest {
    pub fn clamp_values(&mut self) {
        if self.upgrade as i32 > self.item.max_upgrade.unwrap_or_default() {
            self.upgrade = self.item.max_upgrade.unwrap_or_default() as u32
        }

        if self.quantity as i32 > self.item.stack_size {
            self.quantity = self.item.stack_size as u32
        }

        if !self.item.available_infusions().contains(&self.infusion) {
            self.infusion = Infusion::Normal
        }
    }

    pub fn spawn(&self) -> anyhow::Result<()> {
        player_loaded_check()?;
        if self.item.scholar_only {
            scholar_check()?;
        }

        itemspawn(
            self.item.id,
            self.item.stack_size,
            self.item.durability.unwrap_or(0),
            self.quantity,
            self.upgrade,
            self.infusion as i32,
        )?;
        Ok(())
    }

    pub fn can_quantity(&self) -> bool {
        self.item.stack_size > 1
    }
    pub fn can_upgrade(&self) -> bool {
        self.item.max_upgrade.is_some()
    }
    pub fn can_infuse(&self) -> bool {
        self.item.infuse_id.is_some()
    }
}

fn itemspawn(
    item_id: i32,
    stack_size: i32,
    durability: i32,
    quantity: u32,
    upgrade: u32,
    infusion: i32,
) -> anyhow::Result<()> {
    let mut args: [u8; 35] = [0x0; 35];
    write_to_slice::<i32>(&mut args, item_args_offsets::CURRENT_QUANTITY, 0)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::STACK_COUNT, 0)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::MAX_QUANTITY, stack_size)?;
    write_to_slice::<i32>(&mut args, item_args_offsets::ITEM_COUNT, 1)?;
    write_to_slice::<u8>(&mut args, item_args_offsets::ADJUST_QUANTITY_FLAG, stack_size > 1)?;

    let item_struct = item_args_offsets::ITEM_STRUCT;
    write_to_slice::<i32>(&mut args, item_struct + item_struct_offsets::ITEM_ID, item_id)?;
    write_to_slice::<f32>(
        &mut args,
        item_struct + item_struct_offsets::DURABILITY,
        durability as f32,
    )?;
    write_to_slice::<u16>(&mut args, item_struct + item_struct_offsets::QUANTITY, quantity)?;
    write_to_slice::<u8>(&mut args, item_struct + item_struct_offsets::UPGRADE, upgrade)?;
    write_to_slice::<u8>(&mut args, item_struct + item_struct_offsets::INFUSION, infusion)?;

    let args_loc = CaveAddress::ItemArgs.addr();
    write_bytes(args_loc, &args)?;

    let item_struct = args_loc + ITEM_STRUCT;
    use item_struct_offsets as off;

    let mut fun = asm_function("item_spawn");
    let mut asm = fun.take_bytes();

    write_addr_to_slice(&mut asm, fun.reloc("game_man_imp"), BasePointer::GameManagerImp)?;
    write_addr_to_slice(
        &mut asm,
        fun.reloc("adjust_quantity_flag"),
        args_loc + ADJUST_QUANTITY_FLAG,
    )?;
    write_addr_to_slice(&mut asm, fun.reloc("item_id"), item_struct + off::ITEM_ID)?;
    write_addr_to_slice(&mut asm, fun.reloc("stack_count"), args_loc + STACK_COUNT)?;
    write_addr_to_slice(&mut asm, fun.reloc("current_quantity"), args_loc + CURRENT_QUANTITY)?;
    write_addr_to_slice(
        &mut asm,
        fun.reloc("fn_current_item_quantity_check"),
        Function::CurrentItemQuantityCheck,
    )?;
    write_addr_to_slice(&mut asm, fun.reloc("quantity"), item_struct + off::QUANTITY)?;
    write_addr_to_slice(&mut asm, fun.reloc("current_quantity"), args_loc + CURRENT_QUANTITY)?;
    write_addr_to_slice(&mut asm, fun.reloc("max_quantity"), args_loc + MAX_QUANTITY)?;
    write_addr_to_slice(&mut asm, fun.reloc("item_count"), args_loc + ITEM_COUNT)?;
    write_addr_to_slice(&mut asm, fun.reloc("item_struct"), item_struct)?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_item_spawn"), Function::ItemSpawn)?;
    write_addr_to_slice(&mut asm, fun.reloc("stack_loc"), CaveAddress::ItemSpawnStack)?;
    write_addr_to_slice(
        &mut asm,
        fun.reloc("fn_build_item_dialogue"),
        Function::BuildItemDialogue,
    )?;
    write_addr_to_slice(&mut asm, fun.reloc("fn_show_item_dialogue"), Function::ShowItemDialogue)?;

    run_custom_function(asm)
}

pub fn mass_spawn(
    category: Categories,
    quantity: u32,
    upgrade: u32,
    infusion: Infusion,
) -> anyhow::Result<()> {
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
        let mut spawn_request = ItemSpawnRequest {
            item: *item,
            quantity,
            upgrade,
            infusion,
        };
        spawn_request.clamp_values();

        if let Err(err) = spawn_request.spawn()
            && !err.is::<ScholarError>()
        {
            return Err(err);
        }
        thread::sleep(Duration::from_millis(8));
    }
    Ok(())
}

impl Item {
    pub fn available_infusions(&self) -> Vec<Infusion> {
        let mut infusions = Vec::new();
        if let Some(infusion_id) = self.infuse_id
            && let Some(flags) = INFUSION_IDS.get(&infusion_id)
        {
            flags.iter().enumerate().for_each(|(idx, val)| {
                if *val == 1 {
                    infusions.push(INFUSIONS[idx]);
                }
            })
        }
        infusions
    }
}
