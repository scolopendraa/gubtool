use {
    crate::appdata::{AppDataError, app_data_dir},
    std::path::PathBuf,
};

pub struct Dll {
    pub name: &'static str,
    pub data: &'static [u8],
}

impl Dll {
    fn std_path(&self) -> Result<PathBuf, AppDataError> {
        let path = app_data_dir()?
            .join("dll")
            .join(format!("{}.dll", self.name));
        Ok(path)
    }

    pub fn exists_on_disk(&self) -> Result<bool, AppDataError> {
        let path = self.std_path()?;

        if !path.exists() {
            return Ok(false);
        }

        let is_same_size = std::fs::metadata(path)?.len() == self.data.len() as u64;
        Ok(is_same_size)
    }

    pub fn write_to_disk(&self) -> Result<(), AppDataError> {
        let path = self.std_path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, self.data)?;
        Ok(())
    }

    pub fn get_win_path_bytes(&self) -> Result<Vec<u8>, AppDataError> {
        #[cfg(windows)]
        let path = self.std_path()?;
        #[cfg(windows)]
        let string = path.to_string_lossy();

        #[cfg(unix)]
        let string = format!("Z:{}", self.std_path()?.to_string_lossy().replace('/', r"\"));

        let bytes = string
            .encode_utf16()
            .chain(std::iter::once(0))
            .flat_map(|unit| unit.to_le_bytes())
            .collect();

        Ok(bytes)
    }

    pub fn inject_by_request(&self) {}
}
