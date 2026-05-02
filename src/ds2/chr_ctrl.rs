use anyhow::{Result, anyhow};

use crate::ds2::{
    mem::{read, write},
    offsets::game_manager_imp::chr_ctrl_offsets,
};

pub type ChrCtrl = Result<u64>;

pub trait ChrCtrlExt {
    fn get_min_hp(&self) -> Result<i32>;
    fn set_min_hp(&self, val: i32) -> Result<()>;
    fn is_no_death(&self) -> Result<bool>;
    fn set_no_death(&self, state: bool) -> Result<()>;
    fn coords(&self) -> Result<[f32; 3]>;

    fn chr_ctrl_pointer(&self) -> Result<u64>;
}

impl ChrCtrlExt for ChrCtrl {
    fn get_min_hp(&self) -> Result<i32> {
        read::<i32>(self.chr_ctrl_pointer()? + chr_ctrl_offsets::min_hp())
    }

    fn set_min_hp(&self, val: i32) -> Result<()> {
        write::<i32>(self.chr_ctrl_pointer()? + chr_ctrl_offsets::min_hp(), val)
    }

    fn is_no_death(&self) -> Result<bool> {
        self.get_min_hp().map(|val| val == 1)
    }

    fn set_no_death(&self, state: bool) -> Result<()> {
        let val = if state { 1 } else { -99999 };
        self.set_min_hp(val)
    }

    fn coords(&self) -> Result<[f32; 3]> {
        read::<[f32; 3]>(self.chr_ctrl_pointer()? + chr_ctrl_offsets::coords())
    }

    fn chr_ctrl_pointer(&self) -> Result<u64> {
        Ok(*self.as_ref().map_err(|e| anyhow!("{e}"))?)
    }
}