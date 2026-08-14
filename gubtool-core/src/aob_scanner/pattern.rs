pub struct AobScan {
    pub name:        &'static str,
    pub pattern:     &'static str,
    pub scan_origin: u64,
    pub offset:      i64,
    pub scan_mode:   AddressingMode,
}

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Absolute,
    Direct32,
    Relative {
        bytes_to_next_instr: i32,
    },
    VfTableRelative {
        table_offset: u64,
    },
}
