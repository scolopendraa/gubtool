pub mod attach;
mod watcher;

use {
    crate::watcher::watch,
    gubtool_core::appdata::AppDataError,
    serde::{Deserialize, Serialize},
    std::path::PathBuf,
};

pub trait Config: Serialize + for<'a> Deserialize<'a> + Default + Clone {
    fn get_path() -> Result<PathBuf, AppDataError>;
    fn read() -> Result<Self, AppDataError>
    where Self: Sized;
    fn write(&self) -> Result<(), AppDataError>;
    fn update<F>(modifier: F) -> Result<(), AppDataError>
    where F: FnOnce(&mut Self);
}

pub fn start_watcher_thread() {
    std::thread::spawn(|| {
        watch().unwrap();
    });
}
