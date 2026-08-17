use {
    crate::{
        mem::*,
        offsets::{code_cave::CaveAddress, module_offsets::Function},
        pointer_cache::ResolvedPtr,
        resources::{bonfires::Bonfire, bosses::Boss},
        utils::player_loaded_check,
    },
    gubtool_core::{
        address::Address,
        sys::ipc::{FfiValue, X86CallingConvention},
    },
};

const DEFAULT_TRANSITION_MODE: u32 = 6;
const DEFAULT_SPAWN_ANIM: u32 = 3;

#[repr(C, packed)]
struct WarpRequest {
    kind:                     u32,
    transition_mode:          u32,
    map_id:                   u32,
    unk_0c:                   i32,
    post_warp_demo_id:        u32,
    spawn_anim:               u32,
    payload:                  Payload,
    quaternion:               [f32; 4],
    pre_warp_demo_id:         u32,
    post_submit_flag:         u8,
    post_submit_special_flag: u8,
    _pad:                     u16,
}

#[repr(C, packed)]
union Payload {
    pos:        [f32; 4],
    payload_id: u32,
}

enum WarpKind {
    Direct           = 0,
    DirectWithOffset = 1,
    MapOnly          = 2,
    Bonfire          = 3,
    EventPoint       = 4,
}

impl Default for WarpRequest {
    fn default() -> Self {
        Self {
            kind:                     0,
            transition_mode:          DEFAULT_TRANSITION_MODE,
            map_id:                   0,
            unk_0c:                   -1,
            post_warp_demo_id:        0,
            spawn_anim:               DEFAULT_SPAWN_ANIM,
            payload:                  Payload {
                payload_id: 0,
            },
            quaternion:               [0.0; 4],
            pre_warp_demo_id:         0,
            post_submit_flag:         0,
            post_submit_special_flag: 0,
            _pad:                     0,
        }
    }
}

impl WarpRequest {
    fn to_array(&self) -> [u8; std::mem::size_of::<Self>()] {
        unsafe { std::mem::transmute_copy(self) }
    }
}

impl Boss {
    pub fn warp(&self) -> anyhow::Result<()> {
        player_loaded_check()?;

        let request = WarpRequest {
            kind: WarpKind::Direct as u32,
            payload: Payload {
                pos: self.pos,
            },
            quaternion: self.quaternion,
            map_id: self.map_id as u32,
            ..Default::default()
        };
        warp(request)?;
        Ok(())
    }
}

impl Bonfire {
    pub fn warp(&self) -> anyhow::Result<()> {
        player_loaded_check()?;

        let request = WarpRequest {
            kind: WarpKind::Bonfire as u32,
            map_id: self.map_id as u32,
            payload: Payload {
                payload_id: self.bonfire_id,
            },
            ..Default::default()
        };
        warp(request)?;
        Ok(())
    }
}

fn warp(request: WarpRequest) -> anyhow::Result<()> {
    write_bytes(CaveAddress::WarpRequestStruct, &request.to_array())?;

    let args = [
        FfiValue::pointer(ResolvedPtr::WarpManager.get()?),
        FfiValue::pointer(CaveAddress::WarpRequestStruct.addr()),
    ];

    run_game_function(Function::Warp, &args, X86CallingConvention::__thiscall)
}
