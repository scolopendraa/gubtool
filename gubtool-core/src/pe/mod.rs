pub mod error;

use {
    crate::{attached::AddressSize, pe::error::ParsePeError},
    pelite::{
        FileMap,
        pe32::Pe as Pe32,
        pe64::{Pe as Pe64, headers::SectionHeaders},
    },
    std::path::PathBuf,
};

pub struct PeParser {
    file_map: FileMap,
}

pub enum PeFile<'a> {
    Pe32(pelite::pe32::PeFile<'a>),
    Pe64(pelite::pe64::PeFile<'a>),
}

impl PeParser {
    pub fn new<T>(path: T) -> Result<Self, ParsePeError>
    where T: Into<PathBuf> {
        let path = path.into();
        let file_map = FileMap::open(&path)?;

        Ok(Self {
            file_map,
        })
    }

    fn pe_file(&self) -> Result<PeFile<'_>, ParsePeError> {
        if let Ok(pe) = pelite::pe64::PeFile::from_bytes(&self.file_map) {
            return Ok(PeFile::Pe64(pe));
        }

        let pe = pelite::pe32::PeFile::from_bytes(&self.file_map)?;

        Ok(PeFile::Pe32(pe))
    }

    pub fn version_info(&self) -> Result<(u16, u16, u16), ParsePeError> {
        let version_info = match self.pe_file()? {
            PeFile::Pe32(pe) => pe.resources()?.version_info()?,
            PeFile::Pe64(pe) => pe.resources()?.version_info()?,
        };

        let version = version_info.fixed().unwrap().dwProductVersion;

        Ok((version.Major, version.Minor, version.Patch))
    }

    pub fn size_of_image(&self) -> Result<u32, ParsePeError> {
        let size = match self.pe_file()? {
            PeFile::Pe32(pe) => pe.optional_header().SizeOfImage,
            PeFile::Pe64(pe) => pe.optional_header().SizeOfImage,
        };

        Ok(size)
    }

    pub fn address_size(&self) -> Result<AddressSize, ParsePeError> {
        let size = match self.pe_file()? {
            PeFile::Pe32(_) => AddressSize::Bits32,
            PeFile::Pe64(_) => AddressSize::Bits64,
        };

        Ok(size)
    }

    pub fn section_headers(&self) -> Result<&SectionHeaders, ParsePeError> {
        let headers = match self.pe_file()? {
            PeFile::Pe32(pe) => pe.section_headers(),
            PeFile::Pe64(pe) => pe.section_headers(),
        };
        Ok(headers)
    }
}
