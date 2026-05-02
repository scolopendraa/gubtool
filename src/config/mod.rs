pub mod preferences;
pub mod ui_state;

use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};


pub trait Config: Serialize + for<'a> Deserialize<'a> + Default + Clone {
    fn get_path() -> Result<PathBuf>;
    fn read() -> Result<Self> where Self: Sized;
    fn write(&self) -> Result<()>;
    fn update<F>(modifier: F) -> Result<()>
    where F: FnOnce(&mut Self);
}