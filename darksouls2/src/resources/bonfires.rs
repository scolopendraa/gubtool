use crate::resources::map_ids::MapId;

pub struct Bonfire {
    pub name:       &'static str,
    pub bonfire_id: u32,
    pub map_id:     MapId,
}

pub const BONFIRES: &[Bonfire; 77] = &[
    Bonfire {
        name:       "Fire Keeper's Dwelling",
        bonfire_id: 2650,
        map_id:     MapId::ThingsBetwixt,
    },
    Bonfire {
        name:       "Far Fire",
        bonfire_id: 4650,
        map_id:     MapId::Majula,
    },
    Bonfire {
        name:       "Crestfallen's Retreat",
        bonfire_id: 10670,
        map_id:     MapId::ForestOfFallenGiants,
    },
    Bonfire {
        name:       "Cardinal Tower",
        bonfire_id: 10655,
        map_id:     MapId::ForestOfFallenGiants,
    },
    Bonfire {
        name:       "Soldier's Rest",
        bonfire_id: 10660,
        map_id:     MapId::ForestOfFallenGiants,
    },
    Bonfire {
        name:       "The Place Unbeknownst",
        bonfire_id: 10675,
        map_id:     MapId::ForestOfFallenGiants,
    },
    Bonfire {
        name:       "Heide's Ruin",
        bonfire_id: 31655,
        map_id:     MapId::HeidesTowerOfFlame,
    },
    Bonfire {
        name:       "Tower of Flame",
        bonfire_id: 31650,
        map_id:     MapId::HeidesTowerOfFlame,
    },
    Bonfire {
        name:       "Cathedral of Blue",
        bonfire_id: 31660,
        map_id:     MapId::HeidesTowerOfFlame,
    },
    Bonfire {
        name:       "Unseen Path to Heide's",
        bonfire_id: 18650,
        map_id:     MapId::NoMansWharf,
    },
    Bonfire {
        name:       "Exile Holding Cells",
        bonfire_id: 16655,
        map_id:     MapId::TheLostBastille,
    },
    Bonfire {
        name:       "McDuff's Workshop",
        bonfire_id: 16670,
        map_id:     MapId::TheLostBastille,
    },
    Bonfire {
        name:       "Servant's Quarters",
        bonfire_id: 16675,
        map_id:     MapId::TheLostBastille,
    },
    Bonfire {
        name:       "Straid's Cell",
        bonfire_id: 16650,
        map_id:     MapId::TheLostBastille,
    },
    Bonfire {
        name:       "The Tower Apart",
        bonfire_id: 16660,
        map_id:     MapId::TheLostBastille,
    },
    Bonfire {
        name:       "The Saltfort",
        bonfire_id: 16685,
        map_id:     MapId::TheLostBastille,
    },
    Bonfire {
        name:       "Upper Ramparts",
        bonfire_id: 16665,
        map_id:     MapId::TheLostBastille,
    },
    Bonfire {
        name:       "Undead Refuge",
        bonfire_id: 23650,
        map_id:     MapId::HuntsmansCorpse,
    },
    Bonfire {
        name:       "Bridge Approach",
        bonfire_id: 23655,
        map_id:     MapId::HuntsmansCorpse,
    },
    Bonfire {
        name:       "Undead Lockaway",
        bonfire_id: 23660,
        map_id:     MapId::HuntsmansCorpse,
    },
    Bonfire {
        name:       "Undead Purgatory",
        bonfire_id: 23665,
        map_id:     MapId::HuntsmansCorpse,
    },
    Bonfire {
        name:       "Poison Pool",
        bonfire_id: 17665,
        map_id:     MapId::HarvestValley,
    },
    Bonfire {
        name:       "The Mines",
        bonfire_id: 17650,
        map_id:     MapId::HarvestValley,
    },
    Bonfire {
        name:       "Lower Earthen Peak",
        bonfire_id: 17655,
        map_id:     MapId::HarvestValley,
    },
    Bonfire {
        name:       "Central Earthen Peak",
        bonfire_id: 17670,
        map_id:     MapId::HarvestValley,
    },
    Bonfire {
        name:       "Upper Earthen Peak",
        bonfire_id: 17675,
        map_id:     MapId::HarvestValley,
    },
    Bonfire {
        name:       "Threshold Bridge",
        bonfire_id: 19655,
        map_id:     MapId::IronKeep,
    },
    Bonfire {
        name:       "Ironhearth Hall",
        bonfire_id: 19650,
        map_id:     MapId::IronKeep,
    },
    Bonfire {
        name:       "Eygil's Idol",
        bonfire_id: 19660,
        map_id:     MapId::IronKeep,
    },
    Bonfire {
        name:       "Belfry Sol Approach",
        bonfire_id: 19665,
        map_id:     MapId::IronKeep,
    },
    Bonfire {
        name:       "Old Akelarre",
        bonfire_id: 29650,
        map_id:     MapId::PathToShadedWoods,
    },
    Bonfire {
        name:       "Ruined Fork Road",
        bonfire_id: 32655,
        map_id:     MapId::ShadedWoods,
    },
    Bonfire {
        name:       "Shaded Ruins",
        bonfire_id: 32660,
        map_id:     MapId::ShadedWoods,
    },
    Bonfire {
        name:       "Gyrm's Respite",
        bonfire_id: 33655,
        map_id:     MapId::DoorsOfPharros,
    },
    Bonfire {
        name:       "Ordeal's End",
        bonfire_id: 33660,
        map_id:     MapId::DoorsOfPharros,
    },
    Bonfire {
        name:       "Royal Army Campsite",
        bonfire_id: 14655,
        map_id:     MapId::BrightstoneCoveTseldora,
    },
    Bonfire {
        name:       "Chapel Threshold",
        bonfire_id: 14660,
        map_id:     MapId::BrightstoneCoveTseldora,
    },
    Bonfire {
        name:       "Lower Brightstone Cove",
        bonfire_id: 14650,
        map_id:     MapId::BrightstoneCoveTseldora,
    },
    Bonfire {
        name:       "Harvel's Resting Place",
        bonfire_id: 34655,
        map_id:     MapId::GraveOfSaints,
    },
    Bonfire {
        name:       "Grave Entrance",
        bonfire_id: 34650,
        map_id:     MapId::GraveOfSaints,
    },
    Bonfire {
        name:       "Upper Gutter",
        bonfire_id: 25665,
        map_id:     MapId::TheGutter,
    },
    Bonfire {
        name:       "Central Gutter",
        bonfire_id: 25655,
        map_id:     MapId::TheGutter,
    },
    Bonfire {
        name:       "Black Gulch Mouth",
        bonfire_id: 25650,
        map_id:     MapId::TheGutter,
    },
    Bonfire {
        name:       "Hidden Chamber",
        bonfire_id: 25660,
        map_id:     MapId::TheGutter,
    },
    Bonfire {
        name:       "King's Gate",
        bonfire_id: 21650,
        map_id:     MapId::DrangleicCastle,
    },
    Bonfire {
        name:       "Forgotten Chamber",
        bonfire_id: 21660,
        map_id:     MapId::DrangleicCastle,
    },
    Bonfire {
        name:       "Under Castle Drangleic",
        bonfire_id: 21665,
        map_id:     MapId::DrangleicCastle,
    },
    Bonfire {
        name:       "Central Castle Drangleic",
        bonfire_id: 21655,
        map_id:     MapId::DrangleicCastle,
    },
    Bonfire {
        name:       "Tower of Prayer",
        bonfire_id: 11650,
        map_id:     MapId::ShrineOfAmana,
    },
    Bonfire {
        name:       "Crumbled Ruins",
        bonfire_id: 11655,
        map_id:     MapId::ShrineOfAmana,
    },
    Bonfire {
        name:       "Rhoy's Resting Place",
        bonfire_id: 11660,
        map_id:     MapId::ShrineOfAmana,
    },
    Bonfire {
        name:       "Rise of the Dead",
        bonfire_id: 11670,
        map_id:     MapId::ShrineOfAmana,
    },
    Bonfire {
        name:       "Undead Crypt Entrance",
        bonfire_id: 24655,
        map_id:     MapId::UndeadCrypt,
    },
    Bonfire {
        name:       "Undead Ditch",
        bonfire_id: 24650,
        map_id:     MapId::UndeadCrypt,
    },
    Bonfire {
        name:       "Foregarden",
        bonfire_id: 15650,
        map_id:     MapId::AldiasKeep,
    },
    Bonfire {
        name:       "Ritual Site",
        bonfire_id: 15655,
        map_id:     MapId::AldiasKeep,
    },
    Bonfire {
        name:       "Dragon Aerie",
        bonfire_id: 27650,
        map_id:     MapId::DragonAerie,
    },
    Bonfire {
        name:       "Shrine Entrance",
        bonfire_id: 27655,
        map_id:     MapId::DragonAerie,
    },
    Bonfire {
        name:       "Sanctum Walk",
        bonfire_id: 35650,
        map_id:     MapId::ShulvaSanctumCity,
    },
    Bonfire {
        name:       "Tower of Prayer",
        bonfire_id: 35685,
        map_id:     MapId::ShulvaSanctumCity,
    },
    Bonfire {
        name:       "Priestess's Chamber",
        bonfire_id: 35655,
        map_id:     MapId::ShulvaSanctumCity,
    },
    Bonfire {
        name:       "Hidden Sanctum Chamber",
        bonfire_id: 35670,
        map_id:     MapId::ShulvaSanctumCity,
    },
    Bonfire {
        name:       "Lair of the Imperfect",
        bonfire_id: 35675,
        map_id:     MapId::ShulvaSanctumCity,
    },
    Bonfire {
        name:       "Sanctum Interior",
        bonfire_id: 35680,
        map_id:     MapId::ShulvaSanctumCity,
    },
    Bonfire {
        name:       "Sanctum Nadir",
        bonfire_id: 35665,
        map_id:     MapId::ShulvaSanctumCity,
    },
    Bonfire {
        name:       "Throne Floor",
        bonfire_id: 36650,
        map_id:     MapId::BrumeTower,
    },
    Bonfire {
        name:       "Upper Floor",
        bonfire_id: 36660,
        map_id:     MapId::BrumeTower,
    },
    Bonfire {
        name:       "Foyer",
        bonfire_id: 36655,
        map_id:     MapId::BrumeTower,
    },
    Bonfire {
        name:       "Lowermost Floor",
        bonfire_id: 36670,
        map_id:     MapId::BrumeTower,
    },
    Bonfire {
        name:       "The Smelter Throne",
        bonfire_id: 36675,
        map_id:     MapId::BrumeTower,
    },
    Bonfire {
        name:       "Iron Passage",
        bonfire_id: 36665,
        map_id:     MapId::BrumeTower,
    },
    Bonfire {
        name:       "Outer Wall",
        bonfire_id: 37650,
        map_id:     MapId::FrozenEleumLoyce,
    },
    Bonfire {
        name:       "Abandoned Dwelling",
        bonfire_id: 37660,
        map_id:     MapId::FrozenEleumLoyce,
    },
    Bonfire {
        name:       "Inner Wall",
        bonfire_id: 37685,
        map_id:     MapId::FrozenEleumLoyce,
    },
    Bonfire {
        name:       "Lower Garrison",
        bonfire_id: 37665,
        map_id:     MapId::FrozenEleumLoyce,
    },
    Bonfire {
        name:       "Expulsion Chamber",
        bonfire_id: 37675,
        map_id:     MapId::FrozenEleumLoyce,
    },
    Bonfire {
        name:       "Grand Cathedral",
        bonfire_id: 37670,
        map_id:     MapId::FrozenEleumLoyce,
    },
];
