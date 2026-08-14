mod macros;
pub mod pattern;
mod resolve;
mod scan;
pub mod scan_error;

use {
    crate::{
        aob_scanner::{pattern::AobScan, scan_error::ScanError},
        attached::{self},
        pe::PeParser,
    },
    std::path::PathBuf,
};

pub struct AobScanner {
    scan:        &'static AobScan,
    location:    ScanLocation,
    constraints: ScanConstraints,
    pattern:     Vec<Option<u8>>,
    exhaustive:  bool,
}

pub enum ScanLocation {
    Disk(FileWithPath),
    Memory,
}

pub struct FileWithPath {
    path: PathBuf,
    file: std::fs::File,
}

struct ScanConstraints {
    start:  u64,
    end:    u64,
    origin: u64,
}

#[derive(Clone, Copy)]
pub enum ScanStrategy<'a> {
    Mem,
    MemExhaustive,
    Disk(&'a PathBuf),
    DiskExhaustive(&'a PathBuf),
}

impl AobScanner {
    pub fn memory(scan: &'static AobScan) -> Result<Self, ScanError> {
        let pattern = parse_ida(scan.pattern)?;
        let location = ScanLocation::Memory;
        let constraints = make_scan_constraints(&location, scan)?;
        Ok(Self {
            scan,
            location,
            constraints,
            pattern,
            exhaustive: false,
        })
    }

    pub fn disk<T>(scan: &'static AobScan, path: T) -> Result<Self, ScanError>
    where T: Into<PathBuf> {
        let pattern = parse_ida(scan.pattern)?;
        let path = path.into();
        let file = std::fs::File::open(&path)?;
        let file_with_path = FileWithPath {
            file,
            path,
        };
        let location = ScanLocation::Disk(file_with_path);
        let constraints = make_scan_constraints(&location, scan)?;
        Ok(Self {
            scan,
            location,
            constraints,
            pattern,
            exhaustive: false,
        })
    }

    pub fn from_strategy(
        scan: &'static AobScan,
        strategy: ScanStrategy,
    ) -> Result<Self, ScanError> {
        match strategy {
            ScanStrategy::Disk(path) => Self::disk(scan, path),
            ScanStrategy::DiskExhaustive(path) => Ok(Self::disk(scan, path)?.make_exhaustive()),
            ScanStrategy::Mem => Self::memory(scan),
            ScanStrategy::MemExhaustive => Ok(Self::memory(scan)?.make_exhaustive()),
        }
    }

    pub fn make_exhaustive(mut self) -> Self {
        self.exhaustive = true;
        self
    }
}

fn parse_ida(ida_pattern: &'static str) -> Result<Vec<Option<u8>>, ScanError> {
    let mut bytes: Vec<Option<u8>> = Vec::new();
    for byte in ida_pattern.split_whitespace() {
        if byte == "?" {
            bytes.push(None)
        } else {
            let b = u8::from_str_radix(byte, 16).map_err(|_| {
                ScanError::ParsePattern {
                    failed_byte: byte,
                }
            })?;
            bytes.push(Some(b))
        }
    }
    Ok(bytes)
}

fn make_scan_constraints(
    location: &ScanLocation,
    scan: &AobScan,
) -> Result<ScanConstraints, ScanError> {
    let start = match location {
        ScanLocation::Disk(_) => 0x0,
        ScanLocation::Memory => attached::module_base(),
    };

    let end = match location {
        ScanLocation::Disk(f) => f.file.metadata().unwrap().len(),
        ScanLocation::Memory => {
            let path = attached::path().unwrap();
            let image_size = PeParser::new(path)?.size_of_image()?;
            attached::module_base() + image_size as u64
        }
    };

    let origin = match location {
        ScanLocation::Disk(_) => scan.scan_origin,
        ScanLocation::Memory => attached::module_base() + scan.scan_origin,
    };

    Ok(ScanConstraints {
        start,
        end,
        origin,
    })
}
