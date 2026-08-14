use {
    crate::aob_scanner::{AobScanner, ScanLocation, scan_error::ScanError},
    std::sync::atomic::{AtomicU64, Ordering},
};

const CHUNK_SIZE: usize = 0x5000;

impl AobScanner {
    pub fn scan(&self) -> Result<u64, ScanError> {
        if self.exhaustive {
            self.scan_ensure_unique()
        } else {
            self.scan_first()
        }
    }

    pub fn scan_all(&self) -> Result<Vec<u64>, ScanError> {
        let mut found = Vec::new();
        let mut offset = self.constraints.start;
        let max_offset = self.constraints.end.saturating_sub(CHUNK_SIZE as u64);
        let step = (CHUNK_SIZE - self.pattern.len() + 1) as u64;

        while offset < self.constraints.end {
            let current = offset.min(max_offset);
            let bytes = self.read_block::<CHUNK_SIZE>(current)?;

            for (i, w) in bytes.windows(self.pattern.len()).enumerate() {
                if self.matches_pattern(w) {
                    found.push(self.resolve_address(offset + i as u64)?);
                }
            }

            if current >= max_offset {
                break;
            }
            offset = current + step
        }

        found.dedup();
        Ok(found)
    }

    pub fn scan_ensure_unique(&self) -> Result<u64, ScanError> {
        let found = self.scan_all()?;

        match found.as_slice() {
            [] => {
                Err(ScanError::NotFound {
                    scan_name: self.scan.name,
                })
            }
            [single] => Ok(*single),
            _ => {
                Err(ScanError::FoundDuplicates {
                    scan_name: self.scan.name,
                    locations: found,
                })
            }
        }
    }

    pub fn scan_first(&self) -> Result<u64, ScanError> {
        let found = AtomicU64::new(u64::MAX);

        let (left, right) = rayon::join(|| self.scan_left(&found), || self.scan_right(&found));

        left?;
        right?;

        let found = found.load(Ordering::Relaxed);
        if found != u64::MAX {
            self.resolve_address(found)
        } else {
            Err(ScanError::NotFound {
                scan_name: self.scan.name,
            })
        }
    }

    fn scan_left(&self, found: &AtomicU64) -> Result<(), ScanError> {
        let mut offset = self.constraints.origin;

        while found.load(Ordering::Relaxed) == u64::MAX && offset > self.constraints.start {
            let next_offset = offset
                .saturating_sub((CHUNK_SIZE - self.pattern.len()) as u64)
                .max(self.constraints.start);

            let bytes = self.read_block::<CHUNK_SIZE>(next_offset)?;

            for (i, w) in bytes.windows(self.pattern.len()).enumerate() {
                if self.matches_pattern(w) {
                    found.store(next_offset + i as u64, Ordering::Relaxed);
                    break;
                }
            }

            offset = next_offset
        }
        Ok(())
    }

    fn scan_right(&self, found: &AtomicU64) -> Result<(), ScanError> {
        let mut offset = self.constraints.origin;
        let step = (CHUNK_SIZE - self.pattern.len() + 1) as u64;
        let max_offset = self.constraints.end.saturating_sub(CHUNK_SIZE as u64);

        while found.load(Ordering::Relaxed) == u64::MAX && offset < self.constraints.end {
            let current = offset.min(max_offset);
            let bytes = self.read_block::<CHUNK_SIZE>(current)?;

            for (i, w) in bytes.windows(self.pattern.len()).enumerate() {
                if self.matches_pattern(w) {
                    found.store(offset + i as u64, Ordering::Relaxed);
                    break;
                }
            }

            if current >= max_offset {
                break;
            }
            offset = current + step
        }
        Ok(())
    }

    fn matches_pattern(&self, slice: &[u8]) -> bool {
        slice
            .iter()
            .zip(&self.pattern)
            .all(|(&slice_byte, &pattern_byte)| pattern_byte.is_none_or(|b| b == slice_byte))
    }

    pub(crate) fn read_block<const N: usize>(&self, offset: u64) -> Result<[u8; N], ScanError> {
        match &self.location {
            ScanLocation::Disk(f) => {
                let mut buf = [0x0; N];
                #[cfg(windows)]
                {
                    use std::os::windows::fs::FileExt;
                    f.file.seek_read(&mut buf, offset)?;
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::FileExt;
                    f.file.read_at(&mut buf, offset)?;
                }
                Ok(buf)
            }
            ScanLocation::Memory => {
                let bytes = crate::sys::read_unsafe::<[u8; N]>(offset)?;
                Ok(bytes)
            }
        }
    }
}
