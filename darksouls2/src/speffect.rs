use {
    crate::{
        mem::{run_game_function, write},
        offsets::{code_cave::CaveAddr, module_offsets::Function},
    },
    gubtool_core::{
        address::Address,
        sys::ipc::{FfiValue, X86CallingConvention},
    },
};

#[repr(C, packed)]
pub struct SpEffect {
    effect_id:   u32,
    quantity:    u32,
    float_value: f32,
    effect_type: u8,
    param_1:     u8,
    param_2:     u8,
    param_3:     u8,
}

pub fn apply_speffect(chr_speffect_ctrl: u64, speffect: SpEffect) -> anyhow::Result<()> {
    write::<SpEffect>(CaveAddr::SpEffectStruct, speffect)?;

    let args = [
        FfiValue::pointer(chr_speffect_ctrl),
        FfiValue::pointer(CaveAddr::SpEffectStruct.addr()),
    ];

    run_game_function(Function::ApplySpeffect, &args, X86CallingConvention::__thiscall)
}

pub const RESTORE_HUMANITY: SpEffect = SpEffect {
    effect_id:   60151000,
    quantity:    1,
    float_value: -1.0,
    effect_type: 0x19,
    param_1:     0x2,
    param_2:     0x0,
    param_3:     0x0,
};

pub const REST: SpEffect = SpEffect {
    effect_id:   110000010,
    quantity:    1,
    float_value: -1.0,
    effect_type: 0x19,
    param_1:     0x2,
    param_2:     0x0,
    param_3:     0x0,
};
