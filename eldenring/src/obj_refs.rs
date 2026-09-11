use {
    crate::{
        chr_ins::ChrIns,
        offsets::{code_cave::CaveAddr, game_data_man, world_chr_man},
        pointer_cache::ResolvedPtr,
    },
    gubtool_core::{
        address::ChainReadExt,
        sys::{
            read_pointer,
            sys_error::{PointerType, SysResult},
        },
    },
    shared::obj_ref::{ObjGuard, ObjRef, ObjUpdater, PlayerPointerType, TargetPointerType},
    std::{fmt::Display, sync::LazyLock},
};

pub struct PlayerUpdater;
type Player = ObjRef<ChrIns, PlayerPointerType, PlayerUpdater>;
pub static PLAYER: LazyLock<Player> = LazyLock::new(|| Player::new());

pub fn player() -> SysResult<ObjGuard<'static, ChrIns>> {
    PLAYER.obj()
}

impl ObjUpdater<ChrIns> for PlayerUpdater {
    fn update(obj_ins: &mut Option<ChrIns>) {
        let ptr = ResolvedPtr::WorldChrMan
            .get()
            .read_ptr(world_chr_man::player_ins());

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

pub struct TorrentUpdater;
type Torrent = ObjRef<ChrIns, TorrentPointerType, TorrentUpdater>;
pub static TORRENT: LazyLock<Torrent> = LazyLock::new(|| Torrent::new());

pub fn torrent() -> SysResult<ObjGuard<'static, ChrIns>> {
    TORRENT.obj()
}

impl ObjUpdater<ChrIns> for TorrentUpdater {
    fn update(obj_ins: &mut Option<ChrIns>) {
        let handle = ResolvedPtr::PlayerGameData
            .get()
            .read_ptr(game_data_man::torrent_handle())
            .unwrap_or_default();

        *obj_ins = ChrIns::from_handle(handle);
    }
}

#[derive(Debug)]
pub struct TorrentPointerType;

impl PointerType for TorrentPointerType {
    fn instance() -> &'static dyn PointerType
    where Self: Sized {
        &TorrentPointerType
    }
}

impl Display for TorrentPointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Torrent instance not found")
    }
}
