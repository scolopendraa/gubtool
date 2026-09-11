use {
    crate::{pointer_cache::ResolvedPtr, resources::ASM},
    gubtool_core::{
        attached::version,
        game_version::DarkSouls3Version::{self, *},
        sys::read,
    },
    thiserror::Error,
};

#[derive(Error, Debug)]
pub(crate) enum DlcError {
    #[error("Requires version 1.08 or above")]
    Version,
    #[error("Requires Ashes of Ariandel")]
    Dlc1,
    #[error("Requires The Ringed City")]
    Dlc2,
}

pub(crate) fn dlc_check(dlc: u8) -> anyhow::Result<()> {
    if dlc == 0 {
        return Ok(())
    }

    if matches!(version::<DarkSouls3Version>(), Some(v) if v < Version1_8_0) {
        return Err(DlcError::Version.into());
    }

    let cs_dlc = ResolvedPtr::CSDlc.get()?;

    if dlc == 1 {
        return if read::<u8>(cs_dlc + 0x11)? != 0x1 {
            Err(DlcError::Dlc1.into())
        } else {
            Ok(())
        }
    }

    if dlc == 2 {
        return if read::<u8>(cs_dlc + 0x12)? != 0x1 {
            Err(DlcError::Dlc2.into())
        } else {
            Ok(())
        }
    }

    unreachable!("invalid dlc")
}

pub fn print_asm_sizes() {
    println!("Dark Souls III");
    ASM.print_function_sizes();
    println!("\n");
}
