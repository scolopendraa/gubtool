pub(crate) const WIN32_PREFFERED_IMAGE_BASE: u64 = 0x400000;
pub(crate) const WIN64_PREFFERED_IMAGE_BASE: u64 = 0x140000000;
// pub(crate) const SHADPS4_BASE: u64 = 0x800000000;

pub trait Address: Copy {
    fn addr(&self) -> u64;

    fn add_offset(&self, offset: u64) -> u64 {
        self.addr().saturating_add(offset)
    }
    fn sub_offset(&self, offset: u64) -> u64 {
        self.addr().saturating_sub(offset)
    }
}

impl Address for u64 {
    fn addr(&self) -> u64 {
        *self
    }
}
