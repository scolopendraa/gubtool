/// Achievement disable functionality.
///
/// This module provides functionality to disable Steam achievements.
/// The implementation writes to the CSTrophy.IsAwardAchievementEnabled flag
/// to prevent achievements from being granted.
///
/// Reference: TarnishedTool implements this the same way by writing to
/// CSTrophy + CSTrophyPlatformImp_forSteam(0x8) + IsAwardAchievementEnabled(0x4C).

use crate::mem::{read, write};
use crate::offsets::module_offsets::BasePointer;
use gubtool_core::address::Address;
use gubtool_core::sys::error::{AccessType, ProcResult, ProcessError};
use std::sync::Mutex;

/// Internal state for the achievement patch.
/// Protected by a Mutex to prevent TOCTOU races between install/uninstall
/// and to store the original flag value for restoration.
struct AchievementPatchState {
    /// Whether the patch is currently installed.
    installed: bool,
    /// The original value of the flag before patching (for restoration).
    original_value: u8,
}

static PATCH_STATE: Mutex<AchievementPatchState> = Mutex::new(AchievementPatchState {
    installed: false,
    original_value: 1,
});

/// Internal helper: read the trophy pointer chain and return the flag pointer.
/// Returns None if the chain is broken.
fn resolve_award_flag_ptr() -> Option<u64> {
    let trophy_base = BasePointer::CSTrophy.addr();
    if trophy_base == 0 {
        return None;
    }
    let trophy_platform = read::<u64>(trophy_base + 0x8).ok()?;

    // Validate pointer - if 0, the chain is broken
    if trophy_platform == 0 {
        return None;
    }

    Some(trophy_platform + 0x4C)
}

/// Internal helper: verify the current flag value in the game matches our
/// expectation. This helps detect if the game has reset the flag (e.g., after
/// a scene change or game reload).
pub(crate) fn verify_patch() -> ProcResult<bool> {
    let flag_ptr = resolve_award_flag_ptr().ok_or_else(|| {
        ProcessError::partial_access(
            AccessType::Read("CSTrophy pointer chain"),
            0,
            0,
        )
    })?;
    
    let current = read::<u8>(flag_ptr)?;
    let state = PATCH_STATE.lock().unwrap();
    
    if state.installed {
        // Patch should be installed: flag should be 0
        Ok(current == 0)
    } else {
        // Patch should not be installed: flag should match original
        Ok(current == state.original_value)
    }
}

/// Install the achievement disable patch.
///
/// This writes `false` to CSTrophy.IsAwardAchievementEnabled, preventing
/// achievements from being granted by the game.
pub fn install_patch() -> ProcResult {
    let mut state = PATCH_STATE.lock().unwrap();
    
    // Check if already installed (thread-safe within lock)
    if state.installed {
        return Ok(());
    }

    let award_flag_ptr = resolve_award_flag_ptr().ok_or_else(|| {
        ProcessError::partial_access(
            AccessType::Read("CSTrophy pointer chain"),
            0,
            0,
        )
    })?;

    // Save the original value for verification and uninstall
    state.original_value = read::<u8>(award_flag_ptr)?;

    // Write 0 to disable achievements
    write::<u8>(award_flag_ptr, 0)?;

    state.installed = true;

    Ok(())
}

/// Uninstall the achievement disable patch.
///
/// This writes `true` back to CSTrophy.IsAwardAchievementEnabled,
/// re-enabling achievements.
pub fn uninstall_patch() -> ProcResult {
    let mut state = PATCH_STATE.lock().unwrap();
    
    // If not installed, nothing to do
    if !state.installed {
        return Ok(());
    }

    // Always re-resolve the pointer chain on uninstall.
    // The CSTrophy module may have been reloaded since install,
    // making any cached pointer stale.
    let award_flag_ptr = resolve_award_flag_ptr().ok_or_else(|| {
        ProcessError::partial_access(
            AccessType::Read("CSTrophy pointer chain"),
            0,
            0,
        )
    })?;

    // Write the original value back
    write::<u8>(award_flag_ptr, state.original_value)?;

    state.installed = false;

    Ok(())
}

/// Check if the achievement patch is currently installed.
pub fn is_installed() -> bool {
    let state = PATCH_STATE.lock().unwrap();
    state.installed
}
