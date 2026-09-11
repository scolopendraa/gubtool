use {
    crate::{offsets::module_offsets::Patch, pointer_cache::ResolvedPtr, resources::ASM},
    gubtool_core::{
        address::ChainReadExt,
        sys::{read, sys_error::SysResult, write, write_bytes},
    },
    shared::{toggle_command, unit_command, value_command},
};

pub(crate) fn disable_logos() -> SysResult {
    let code = ASM.get_function("logo_patch");
    write_bytes(Patch::DisableLogos, &code.bytes)
}

unit_command!(Quitout {
    Ok(ResolvedPtr::MenuMan.get()
        .add_offset(0x250)
        .write::<u8>(0x1)?)
});

toggle_command!(DisableCameraFollow {
    is: {
        read::<u8>(Patch::CameraRotation)
            .map(|val| val ==  0x90)
    }

    set(state): {
        let bytes = match state {
            true => [0x90; 8],
            false => [0x66, 0x0f, 0x7f, 0xa6, 0x40, 0x01, 0x00, 0x00],
        };

        Ok(write::<[u8; 8]>(Patch::CameraRotation, bytes)?)
    }
});

value_command!(ClearCount, u32 {
    get: {
        ResolvedPtr::GameDataMan
            .get()
            .add_offset(0x78)
            .read::<u32>()
    }

    set(val): {
        Ok(ResolvedPtr::GameDataMan
            .get()
            .add_offset(0x78)
            .write::<u32>(val)?)
    }
});

value_command!(GameSpeed, f32 {
    get: {
        read::<f32>(Patch::GameSpeed)
    }

    set(val): {
        Ok(write::<f32>(Patch::GameSpeed, val)?)
    }
});

value_command!(FpsCap, f32 {
    get: {
        let unlocked = ResolvedPtr::SprjFlipperImp
            .get()
            .add_offset(0x358)
            .read::<u8>()
            .map(|val| val != 0)?;

        if !unlocked {
            return Ok(60_f32)
        }

        ResolvedPtr::SprjFlipperImp
            .get()
            .add_offset(0x354)
            .read::<f32>()
    }

    set(val): {
        ResolvedPtr::SprjFlipperImp
            .get()
            .add_offset(0x358)
            .write::<u8>(1)?;
        ResolvedPtr::SprjFlipperImp
            .get()
            .add_offset(0x354)
            .write::<f32>(val)?;
        Ok(())
    }
});

toggle_command!(AccessFullShops {
    is: {
        read::<[u8; 2]>(Patch::FullShop)
            .map(|val| val != [0x74, 0x14])
    }

    set(state): {
        let bytes = match state {
            true => [0x90; 2],
            false => [0x74, 0x14],
        };

        Ok(write::<[u8; 2]>(Patch::FullShop, bytes)?)
    }
});
