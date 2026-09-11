use std::sync::LazyLock;

use gubtool_core::sys::{read_pointer, sys_error::SysResult};
use shared::obj_ref::{ObjGuard, ObjRef, ObjUpdater, PlayerPointerType, TargetPointerType};

use crate::{chr_ins::ChrIns, offsets::code_cave::CaveAddr, pointer_cache::ResolvedPtr};

pub struct PlayerUpdater;
type Player = ObjRef<ChrIns, PlayerPointerType, PlayerUpdater>;
pub static PLAYER: LazyLock<Player> = LazyLock::new(|| Player::new());

pub fn player() -> SysResult<ObjGuard<'static, ChrIns>> {
    PLAYER.obj()
}

impl ObjUpdater<ChrIns> for PlayerUpdater {
    fn update(obj_ins: &mut Option<ChrIns>) {
        let ptr = ResolvedPtr::PlayerIns.get();

        match ptr {
            Ok(pointer) => {
                if pointer.address != 0 {
                    *obj_ins = Some(ChrIns::new(pointer));
                } else {
                    *obj_ins = None;
                }
            }
            Err(_) => {
                *obj_ins = None;
            }
        }
    }
}

pub struct TargetUpdater;
type Target = ObjRef<ChrIns, TargetPointerType, TargetUpdater>;
pub static TARGET: LazyLock<Target> = LazyLock::new(|| Target::new());

pub fn target() -> SysResult<ObjGuard<'static, ChrIns>> {
    TARGET.obj()
}

impl ObjUpdater<ChrIns> for TargetUpdater {
    fn update(obj_ins: &mut Option<ChrIns>) {
        match read_pointer(CaveAddr::SavedTargetPointer) {
            Ok(new_target) => {
                let is_same_target = obj_ins.as_ref().is_some_and(|chr| chr.base == new_target);

                if !is_same_target {
                    if new_target != 0 {
                        *obj_ins = Some(ChrIns::new(new_target))
                    } else {
                        *obj_ins = None;
                    }
                }
            }
            Err(_) => {
                *obj_ins = None;
            }
        }
    }
}