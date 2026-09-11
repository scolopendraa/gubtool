use {
    crate::{
        chr_ctrl::ChrCtrl,
        offsets::{code_cave::CaveAddr, game_manager_imp},
        pointer_cache::ResolvedPtr,
    },
    gubtool_core::{
        address::ChainReadExt,
        sys::{read_pointer, sys_error::SysResult, write_address},
    },
    shared::obj_ref::{ObjGuard, ObjRef, ObjUpdater, PlayerPointerType, TargetPointerType},
    std::sync::LazyLock,
};

pub struct PlayerUpdater;
type Player = ObjRef<ChrCtrl, PlayerPointerType, PlayerUpdater>;
pub static PLAYER: LazyLock<Player> = LazyLock::new(|| Player::new());

pub fn player() -> SysResult<ObjGuard<'static, ChrCtrl>> {
    PLAYER.obj()
}

impl ObjUpdater<ChrCtrl> for PlayerUpdater {
    fn update(obj_ins: &mut Option<ChrCtrl>) {
        let ptr = ResolvedPtr::GameManagerImp
            .get()
            .read_ptr(game_manager_imp::PLAYER_CTRL);
        match ptr {
            Ok(pointer) => {
                if pointer != 0x0 {
                    *obj_ins = Some(ChrCtrl::new(pointer));
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
type Target = ObjRef<ChrCtrl, TargetPointerType, TargetUpdater>;
pub static TARGET: LazyLock<Target> = LazyLock::new(|| Target::new());

pub fn target() -> SysResult<ObjGuard<'static, ChrCtrl>> {
    TARGET.obj()
}

impl ObjUpdater<ChrCtrl> for TargetUpdater {
    fn update(obj_ins: &mut Option<ChrCtrl>) {
        match read_pointer(CaveAddr::SavedTargetPointer) {
            Ok(new_target) => {
                let is_same_target = obj_ins.as_ref().is_some_and(|chr| chr.base == new_target);

                if !is_same_target {
                    if new_target != 0 {
                        *obj_ins = Some(ChrCtrl::new(new_target));
                    } else {
                        *obj_ins = None;
                    }
                }
            }
            Err(_) => {
                *obj_ins = None;
            }
        }

        if let Some(chr_ctrl) = obj_ins
            && !chr_ctrl.is_valid_chr().unwrap_or(false)
        {
            let _ = write_address(CaveAddr::SavedTargetPointer, 0x0);
            *obj_ins = None;
        }
    }
}

pub fn clear_target() {
    let _ = write_address(CaveAddr::SavedTargetPointer, 0x0);
    *TARGET.obj_ins.lock().unwrap() = None;
}
