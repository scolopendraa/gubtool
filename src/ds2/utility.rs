use anyhow::Result;

use crate::ds2::{mem::*, offsets::game_manager_imp};

pub fn quitout() -> Result<()> {
    read::<u64>(game_manager_imp::base())
        .and_then(|addr| write::<u8>(addr + game_manager_imp::quitout(), 6))
}
