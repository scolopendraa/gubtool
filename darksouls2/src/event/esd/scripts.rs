use {
    crate::{
        event::{
            esd::{EsdEventScript, event_commands::*},
            get_event_flag,
            get_obj_state,
        },
        pointer_cache::ResolvedPtr,
        resources::map_ids::MapId,
    },
    gubtool_core::sys::sys_error::SysResult,
    shared::{command::ToggleCommand, toggle_command},
};

const MYTHA_POISON_WATER_ON: EsdEventScript = EsdEventScript {
    map_id:    MapId::HarvestValley,
    functions: &[
        SetEventFlag(117000055, 1),
        ChangeObjState(10173000, 20),
        ChangeObjState(10173001, 20),
        ChangeObjState(10173002, 20),
        ChangeObjState(10173003, 20),
        ChangeObjState(10173004, 20),
        SetGroundMaterial(30, 240, 1),
        SetGroundMaterial(21, 240, 1),
        SetGroundMaterial(20, 240, 1),
        SetGroundMaterial(60, 240, 1),
        SetGroundMaterial(50, 240, 1),
    ],
};

const MYTHA_POISON_WATER_OFF: EsdEventScript = EsdEventScript {
    map_id:    MapId::HarvestValley,
    functions: &[
        SetEventFlag(117000055, 0),
        InitializeObj(10173000),
        InitializeObj(10173001),
        InitializeObj(10173002),
        InitializeObj(10173003),
        InitializeObj(10173004),
        SetGroundMaterial(30, 240, 0),
        SetGroundMaterial(21, 240, 0),
        SetGroundMaterial(20, 240, 0),
        SetGroundMaterial(60, 240, 0),
        SetGroundMaterial(50, 240, 0),
    ],
};

const MOVE_FLEXILE_SHIP: EsdEventScript = EsdEventScript {
    map_id:    MapId::NoMansWharf,
    functions: &[
        AttachObjToObj(10182000, 150, 10182002),
        SetEventFlag(118000010, 1),
        ChangeObjState(10182002, 70),
        SetMapPartDisplay(1, 1),
        SetHitEnabled(4, 1),
        SetHitEnabled(3, 1),
        ChangeObjState(10182000, 21),
        SetPointLightEnabled(10180030, 1, 0),
        SetPointLightEnabled(10180040, 1, 0),
        PlaySfxAtPoint(1000),
        PlaySfxAtPoint(1010),
        DeleteNavimeshAttribute(100000, 2),
    ],
};

const MOVE_FLEXILE_SHIP_BACK: EsdEventScript = EsdEventScript {
    map_id:    MapId::NoMansWharf,
    functions: &[
        SetEventFlag(118000010, 0),
        InitializeObj(10182002),
        SetMapPartDisplay(1, 0),
        SetHitEnabled(4, 0),
        SetHitEnabled(3, 0),
        InitializeObj(10182000),
        SetPointLightEnabled(10180030, 0, 0),
        SetPointLightEnabled(10180040, 0, 0),
        AddNavimeshAttribute(100000, 2),
    ],
};

const OPEN_BELFRY_GATE: EsdEventScript = EsdEventScript {
    map_id:    MapId::TheLostBastille,
    functions: &[
        ChangeObjState(10161051, 20),
        DeleteNavimeshAttribute(400000, 2),
        DisableWhiteDoorKeyGuide(10160620, 0),
    ],
};

const CLOSE_BELFRY_GATE: EsdEventScript = EsdEventScript {
    map_id:    MapId::TheLostBastille,
    functions: &[
        InitializeObj(10161051),
        AddNavimeshAttribute(400000, 2),
        DisableWhiteDoorKeyGuide(10160620, 1),
    ],
};

toggle_command!(FlexileShipDocked {
    is: {
        get_event_flag(118000010)
    }

    set(state): {
        let script = match state {
            true => MOVE_FLEXILE_SHIP,
            false => MOVE_FLEXILE_SHIP_BACK,
        };
        script.execute()
    }
});

toggle_command!(MythaPoisonDrained {
    is: {
        get_event_flag(117000055)
    }

    set(state): {
        let script = match state {
            true => MYTHA_POISON_WATER_ON,
            false => MYTHA_POISON_WATER_OFF,
        };
        script.execute()
    }
});

toggle_command!(BelfryGargoylesGateOpen {
    is: {
        let state = ResolvedPtr::BelfryGateStateActCtrl.get()?;
        let obj_state = get_obj_state(state)?;
        Ok(obj_state == 20)
    }

    set(state): {
        let script = match state {
            true => OPEN_BELFRY_GATE,
            false => CLOSE_BELFRY_GATE,
        };
        script.execute()
    }
});
