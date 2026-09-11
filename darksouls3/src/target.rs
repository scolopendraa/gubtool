use {
    crate::{
        chr_ins, offsets::{code_cave::CaveAddr, module_offsets::Hook}, resources::ASM,
    }, assemble::patch::DWORD, gubtool_core::{
        address::{Address, POINTER}, slice_ops::write_to_slice, sys::{install_hook, read, write_bytes},
    }, shared::toggle_command,
};

const TARGET_HOOK_BYTES_ORIGINAL: [u8; 7] = [0x48, 0x8b, 0x8f, 0, 0, 0, 0];
toggle_command!(SaveTargetHook {
    is: {
        read::<u8>(Hook::LockedTargetPointer)
            .map(|val| val != 0x48)
    }

    set(state): {
        if state {
            let mut fun = ASM.get_function("save_target_hook");
            fun.patch::<POINTER>("saved_pointer_loc", CaveAddr::SavedTargetPointer);
            fun.patch::<DWORD>("modules_offset", chr_ins::modules_offset());
            fun.patch_rel32("hook_loc", CaveAddr::SaveTargetHook, Hook::LockedTargetPointer.add(7), 4);
            install_hook(&fun.bytes, CaveAddr::SaveTargetHook, Hook::LockedTargetPointer, 7)?;
        } else {
            let mut bytes = TARGET_HOOK_BYTES_ORIGINAL;
            write_to_slice::<i32>(&mut bytes, 3, chr_ins::modules_offset())?;
            write_bytes(Hook::LockedTargetPointer, &TARGET_HOOK_BYTES_ORIGINAL)?;
        }
        Ok(())
    }
});
