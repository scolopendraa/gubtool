#![allow(non_snake_case)]
use crate::event::esd::EventCommand;

pub(super) const fn SetEventFlag(param_1: i32, param_2: i32) -> EventCommand {
    EventCommand::new(130101, &[param_1, param_2])
}

pub(super) const fn InitializeObj(param_1: i32) -> EventCommand {
    EventCommand::new(131652, &[param_1])
}

pub(super) const fn ChangeObjState(param_1: i32, param_2: i32) -> EventCommand {
    EventCommand::new(131636, &[param_1, param_2])
}

pub(super) const fn AttachObjToObj(param_1: i32, param_2: i32, param_3: i32) -> EventCommand {
    EventCommand::new(131641, &[param_1, param_2, param_3])
}

pub(super) const fn AddNavimeshAttribute(param_1: i32, param_2: i32) -> EventCommand {
    EventCommand::new(132131, &[param_1, param_2])
}

pub(super) const fn DeleteNavimeshAttribute(param_1: i32, param_2: i32) -> EventCommand {
    EventCommand::new(132132, &[param_1, param_2])
}

pub(super) const fn DisableWhiteDoorKeyGuide(param_1: i32, param_2: i32) -> EventCommand {
    EventCommand::new(131622, &[param_1, param_2])
}

pub(super) const fn SetMapPartDisplay(param_1: i32, param_2: i32) -> EventCommand {
    EventCommand::new(132154, &[param_1, param_2])
}

pub(super) const fn SetHitEnabled(param_1: i32, param_2: i32) -> EventCommand {
    EventCommand::new(132153, &[param_1, param_2])
}

pub(super) const fn SetPointLightEnabled(param_1: i32, param_2: i32, param_3: i32) -> EventCommand {
    EventCommand::new(132101, &[param_1, param_2, param_3])
}

pub(super) const fn PlaySfxAtPoint(param_1: i32) -> EventCommand {
    EventCommand::new(131501, &[param_1])
}

pub(super) const fn SetGroundMaterial(param_1: i32, param_2: i32, param_3: i32) -> EventCommand {
    EventCommand::new(132161, &[param_1, param_2, param_3])
}

pub(super) const fn CompareObjStateId(param_1: i32, param_2: i32, param_3: i32) -> EventCommand {
    EventCommand::new(131653, &[param_1, param_2, param_3])
}
