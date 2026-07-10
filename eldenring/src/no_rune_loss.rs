//! No Rune Loss on Death feature.
//!
//! This module implements a code-patching approach to prevent rune loss on death,
//! following the TarnishedTool reference implementation. Instead of data manipulation,
//! it patches the game's rune-dropping code with a jump instruction that bypasses
//! the rune loss logic.
//!
//! The patch replaces 6 bytes at the target address with:
//! - `E9 <relative_offset>` (5-byte relative jump)
//! - `90` (NOP padding to fill the original instruction size)
//!
//! Thread safety is ensured through a single `Mutex` that protects both the
//! installed state and the saved original bytes, preventing TOCTOU races.

use crate::mem::{read_bytes, write_bytes};
use crate::offsets::module_offsets::Patch as ModulePatch;
use gubtool_core::address::Address;
use gubtool_core::sys::error::ProcResult;
use std::sync::Mutex;

/// Internal state for the no-rune-loss patch.
/// Protected by a Mutex to prevent TOCTOU races between install/uninstall.
struct NoRuneLossState {
    /// Whether the patch is currently installed.
    installed: bool,
    /// Saved original bytes at the patch location (6 bytes).
    /// Used to restore the original code when disabling the feature.
    original_bytes: Option<[u8; 6]>,
}

/// Global state for the no-rune-loss feature.
static STATE: Mutex<NoRuneLossState> = Mutex::new(NoRuneLossState {
    installed: false,
    original_bytes: None,
});

/// Install the no-rune-loss patch at the version-specific offset.
/// This replaces 6 bytes with a jump instruction that bypasses the rune loss logic.
pub fn install_patch() -> ProcResult {
    let mut state = STATE.lock().unwrap();
    
    if state.installed {
        // Already installed
        return Ok(());
    }

    let patch_addr = ModulePatch::NoRuneLossOnDeath.addr();

    if patch_addr == 0 {
        return Err(gubtool_core::sys::error::ProcessError::partial_access(
            gubtool_core::sys::error::AccessType::Write(
                gubtool_core::sys::error::WriteType::Type("no_rune_loss_on_death"),
            ),
            patch_addr as usize,
            6,
        ));
    }

    // Read the original 6 bytes at the patch location
    let bytes = read_bytes(patch_addr, 6)?;
    let original: [u8; 6] = bytes.try_into().map_err(|_| {
        gubtool_core::sys::error::ProcessError::partial_access(
            gubtool_core::sys::error::AccessType::Read("no_rune_loss_on_death"),
            patch_addr as usize,
            6,
        )
    })?;

    // Save the original bytes for later restoration
    state.original_bytes = Some(original);

    // Transform the existing instruction into an unconditional JMP.
    // The original instruction at the patch location is a conditional jump
    // (e.g., JNE rel32: 0F 85 xx xx xx xx) that skips over the rune loss logic.
    // We change byte 0 to 0xE9 (JMP rel32), reuse the existing offset from
    // bytes 2-5 (adding 1 to land on the instruction right after the original
    // target), and set byte 5 to NOP. This matches the TarnishedTool approach.
    let mut patch_bytes = original;
    patch_bytes[0] = 0xE9; // JMP rel32

    // Read the existing offset from bytes 2-5 (where the original instruction
    // stores its relative target), add 1 to jump past the original target,
    // and write it to bytes 1-4 (where JMP rel32 expects the offset).
    let existing_offset = i32::from_le_bytes([
        original[2], original[3], original[4], original[5],
    ]);
    let new_offset = existing_offset + 1;
    patch_bytes[1..5].copy_from_slice(&new_offset.to_le_bytes());
    patch_bytes[5] = 0x90; // NOP padding

    // Write the patch bytes
    write_bytes(patch_addr, &patch_bytes)?;

    // Mark as installed after successful write
    state.installed = true;

    Ok(())
}

/// Uninstall the no-rune-loss patch and restore the original bytes.
pub fn uninstall_patch() -> ProcResult {
    let mut state = STATE.lock().unwrap();
    
    if !state.installed {
        // Already uninstalled
        return Ok(());
    }

    let Some(bytes) = state.original_bytes else {
        state.installed = false;
        return Ok(());
    };

    let patch_addr = ModulePatch::NoRuneLossOnDeath.addr();
    if patch_addr != 0 {
        write_bytes(patch_addr, &bytes)?;
    }

    // Clear saved bytes and mark as uninstalled only after successful restore.
    // This ensures original bytes are preserved if the write fails.
    state.original_bytes = None;
    state.installed = false;

    Ok(())
}

/// Check if the no-rune-loss feature is currently enabled.
pub fn is_enabled() -> bool {
    let state = STATE.lock().unwrap();
    state.installed
}

/// Toggle the no-rune-loss feature.
/// Installs the patch if disabled, uninstalls if enabled.
pub fn toggle() -> ProcResult {
    if is_enabled() {
        uninstall_patch()
    } else {
        install_patch()
    }
}
