use {
    crate::{
        mem::*,
        offsets::{
            code_cave::CaveAddr,
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
    gubtool_core::address::POINTER,
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

#[repr(C, packed)]
struct ItemArgs {
    adjust_quantity_flag: u32,
    current_quantity:     i32,
    max_quantity:         i32,
    item_count:           i32,
    stack_count:          i32,
    _item_struct_start:   u32,
    item_id:              i32,
    durability:           f32,
    quantity:             u16,
    upgrade:              u8,
    infusion:             u8,
}

fn itemspawn(
    item_id: i32,
    stack_size: i32,
    durability: i32,
    quantity: u32,
    upgrade: u32,
    infusion: i32,
) -> anyhow::Result<()> {
    let args = ItemArgs {
        adjust_quantity_flag: (stack_size > 1) as u32,
        current_quantity: 0,
        max_quantity: stack_size,
        item_count: 1,
        stack_count: 0,
        _item_struct_start: 0,
        item_id,
        durability: durability as f32,
        quantity: quantity as u16,
        upgrade: upgrade as u8,
        infusion: infusion as u8,
    };

    write::<ItemArgs>(CaveAddr::ItemArgs, args)?;

    let mut fun = asm_function("item_spawn");

    fun.patch::<POINTER>("game_man_imp", BasePointer::GameManagerImp);
    fun.patch::<POINTER>("item_args", CaveAddr::ItemArgs);
    fun.patch::<POINTER>("fn_current_item_quantity_check", Function::CurrentItemQuantityCheck);
    fun.patch::<POINTER>("fn_item_spawn", Function::ItemSpawn);
    fun.patch::<POINTER>("stack_loc", CaveAddr::ItemSpawnStack);
    fun.patch::<POINTER>("fn_build_item_dialogue", Function::BuildItemDialogue);
    fun.patch::<POINTER>("fn_show_item_dialogue", Function::ShowItemDialogue);

    run_custom_function(fun)
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
