use {
    crate::{
        address::{WIN32_PREFFERED_IMAGE_BASE, WIN64_PREFFERED_IMAGE_BASE},
        aob_scanner::{AobScanner, ScanError, ScanLocation, pattern::AddressingMode},
        attached::{self, AddressSize},
        pe::{PeParser, error::ParsePeError},
    },
    std::path::PathBuf,
};

impl AobScanner {
    pub(super) fn resolve_address(&self, mut offset: u64) -> Result<u64, ScanError> {
        offset = offset.strict_add_signed(self.scan.offset);

        let resolved = match &self.location {
            ScanLocation::Disk(f) => {
                let file_offset = offset;
                let rva = file_offset_to_rva(&f.path, file_offset)?;

                match self.scan.scan_mode {
                    AddressingMode::Absolute => rva,
                    AddressingMode::Direct32 => {
                        (self.read_u32(file_offset)? as u64)
                            .saturating_sub(WIN32_PREFFERED_IMAGE_BASE)
                    }
                    AddressingMode::Relative {
                        bytes_to_next_instr,
                    } => {
                        let rel_offset = self.read_i32(file_offset)?;

                        rva.checked_add_signed(rel_offset as i64 + bytes_to_next_instr as i64)
                            .ok_or(ScanError::OverflowRelative {
                                scan_name: self.scan.name,
                            })?
                    }
                    AddressingMode::VfTableRelative {
                        table_offset,
                    } => {
                        let rel_offset = self.read_i32(file_offset)?;

                        let table = rva.checked_add_signed(rel_offset as i64 + 4).ok_or(
                            ScanError::OverflowRelative {
                                scan_name: self.scan.name,
                            },
                        )?;

                        let function_pointer = rva_to_file_offset(&f.path, table + table_offset)?;

                        let image = PeParser::new(&f.path)?;
                        let address_size = image.address_size()?;

                        match address_size {
                            AddressSize::Bits32 => {
                                (self.read_u32(function_pointer)? as u64)
                                    .saturating_sub(WIN32_PREFFERED_IMAGE_BASE)
                            }
                            AddressSize::Bits64 => {
                                self.read_u64(function_pointer)?
                                    .saturating_sub(WIN64_PREFFERED_IMAGE_BASE)
                            }
                        }
                    }
                }
            }

            ScanLocation::Memory => {
                let absolute_off = match self.scan.scan_mode {
                    AddressingMode::Absolute => offset,
                    AddressingMode::Direct32 => self.read_u32(offset)? as u64,
                    AddressingMode::Relative {
                        bytes_to_next_instr,
                    } => {
                        let rel_offset = self.read_i32(offset)?;

                        offset
                            .checked_add_signed(rel_offset as i64 + bytes_to_next_instr as i64)
                            .ok_or(ScanError::OverflowRelative {
                                scan_name: self.scan.name,
                            })?
                    }
                    AddressingMode::VfTableRelative {
                        table_offset,
                    } => {
                        let rel_offset = self.read_i32(offset)?;

                        let table = offset.checked_add_signed(rel_offset as i64 + 4).ok_or(
                            ScanError::OverflowRelative {
                                scan_name: self.scan.name,
                            },
                        )?;

                        let function_pointer = table + table_offset;

                        match attached::is_32() {
                            true => self.read_u32(function_pointer)? as u64,
                            false => self.read_u64(function_pointer)?,
                        }
                    }
                };
                absolute_off.saturating_sub(attached::module_base())
            }
        };

        Ok(resolved)
    }
}

fn file_offset_to_rva(path: &PathBuf, file_offset: u64) -> Result<u64, ParsePeError> {
    let image = PeParser::new(path)?;
    let headers = image.section_headers()?;

    for section in headers {
        let start = section.PointerToRawData as u64;
        let end = start + section.SizeOfRawData as u64;

        if file_offset >= start && file_offset < end {
            return Ok(section.VirtualAddress as u64 + (file_offset - start));
        }
    }

    panic!("Found offset is not in any section");
}

fn rva_to_file_offset(path: &PathBuf, rva: u64) -> Result<u64, ParsePeError> {
    let image = PeParser::new(path)?;
    let headers = image.section_headers()?;

    for section in headers {
        let virtual_start = section.VirtualAddress as u64;
        let virtual_end = virtual_start + section.SizeOfRawData as u64;

        if rva >= virtual_start && rva < virtual_end {
            return Ok(section.PointerToRawData as u64 + (rva - virtual_start));
        }
    }

    panic!("RVA is not in any section");
}

impl AobScanner {
    fn read_i32(&self, offset: u64) -> Result<i32, ScanError> {
        let bytes = self.read_block::<4>(offset)?;
        Ok(i32::from_le_bytes(bytes))
    }
    fn read_u32(&self, offset: u64) -> Result<u32, ScanError> {
        let bytes = self.read_block::<4>(offset)?;
        Ok(u32::from_le_bytes(bytes))
    }
    fn read_u64(&self, offset: u64) -> Result<u64, ScanError> {
        let bytes = self.read_block::<8>(offset)?;
        Ok(u64::from_le_bytes(bytes))
    }
}
