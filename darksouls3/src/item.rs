use {
    crate::{
        mem::run_custom_function,
        offsets::{
            code_cave::CaveAddr,
            module_offsets::{BasePointer, Function},
        },
        resources::{
            ASM,
            items::{
                Item,
                ItemCategory,
                armor::ARMOR,
                arrows::ARROWS,
                consumables::CONSUMABLES,
                gestures::GESTURES,
                infusions::Infusion,
                key_items::KEY_ITEMS,
                rings::RINGS,
                spells::SPELLS,
                upgrade_materials::UPGRADE_MATERIALS,
                weapons::WEAPONS,
            },
        },
        utils::{DlcError, dlc_check},
    },
    gubtool_core::{address::POINTER, sys::write},
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
        let item = self.item;

        let max_upgrade = match item.upgrade_type {
            Some(1) => 10,
            Some(2) => 5,
            _ => 0,
        };
        self.upgrade = self.upgrade.min(max_upgrade);

        self.quantity = self.quantity.min(self.max_quantity());

        if !item.infusable {
            self.infusion = Infusion::Normal
        }
    }

    pub fn spawn(&mut self) -> anyhow::Result<()> {
        let item = self.item;

        dlc_check(item.dlc)?;

        self.clamp_values();

        let item_id = item.id + self.upgrade + self.infusion as u32;
        itemspawn(item_id, self.quantity, self.max_quantity(), item.stack_size > 1)
    }

    pub fn can_quantity(&self) -> bool {
        self.item.stack_size > 1
    }

    pub fn can_upgrade(&self) -> bool {
        self.item.upgrade_type.is_some()
    }

    pub fn can_infuse(&self) -> bool {
        self.item.infusable
    }

    fn max_quantity(&self) -> u32 {
        let item = self.item;
        if item.max_storage > 1 {
            item.max_storage + item.stack_size
        } else {
            item.stack_size
        }
    }
}

#[repr(C, packed)]
struct ItemArgs {
    adjust_quantity:    u32,
    max_quantity:       u32,
    _item_struct_start: u32,
    item_id:            u32,
    quantity:           u32,
    durability:         u32,
}

fn itemspawn(
    item_id: u32,
    quantity: u32,
    max_quantity: u32,
    adjust_quantity: bool,
) -> anyhow::Result<()> {
    let bytes = ItemArgs {
        adjust_quantity:    adjust_quantity as u32,
        max_quantity:       max_quantity,
        _item_struct_start: 1,
        item_id:            item_id,
        quantity:           quantity,
        durability:         100,
    };
    write::<ItemArgs>(CaveAddr::ItemArgs, bytes)?;

    let mut fun = ASM.get_function("item_spawn");

    fun.patch::<POINTER>("item_args", CaveAddr::ItemArgs);
    fun.patch::<POINTER>("game_data_man", BasePointer::GameDataMan);
    fun.patch::<POINTER>("fn_get_item_quantity", Function::GetItemQuantity);
    fun.patch::<POINTER>("map_item_man", BasePointer::MapItemMan);
    fun.patch::<POINTER>("output", CaveAddr::ItemSpawnOutput);
    fun.patch::<POINTER>("fn_item_spawn", Function::ItemSpawn);

    run_custom_function(fun)
}

pub fn mass_spawn(
    category: ItemCategory,
    quantity: u32,
    upgrade: u32,
    infusion: Infusion,
) -> anyhow::Result<()> {
    let items: &'static [Item] = match category {
        ItemCategory::Armor => &ARMOR,
        ItemCategory::Arrows => &ARROWS,
        ItemCategory::Consumables => &CONSUMABLES,
        ItemCategory::Gestures => &GESTURES,
        ItemCategory::KeyItems => &KEY_ITEMS,
        ItemCategory::Rings => &RINGS,
        ItemCategory::Spells => &SPELLS,
        ItemCategory::UpgradeMaterials => &UPGRADE_MATERIALS,
        ItemCategory::Weapons => &WEAPONS,
    };

    for item in items {
        let mut request = ItemSpawnRequest {
            item: *item,
            quantity,
            upgrade,
            infusion,
        };

        if let Err(err) = request.spawn()
            && !(err.is::<DlcError>())
        {
            return Err(err);
        }
    }
    Ok(())
}
