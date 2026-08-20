use {
    crate::{
        attached::{is_32, module_base},
        slice_ops::read_from_slice,
        sys::{read_unsafe, sys_error::SysError, write_unsafe},
    },
    arboard::Clipboard,
    pelite::Pod,
    std::{
        collections::HashMap,
        time::{Duration, Instant},
    },
};

pub const READ_SIZE: usize = 0x1000;
const HIGHLIGHT_DURATION: Duration = Duration::from_millis(2500);

pub struct MemoryViewer {
    pub base_address:       u64,
    pub highlighted_offset: u64,
    pub read_successful:    bool,
    pub bytes:              [u8; READ_SIZE],
    pub changed_highlights: HashMap<u64, Instant>,
    pub copied_highlights:  HashMap<u64, Instant>,
    clipboard:              Option<Clipboard>,
    jump_history:           Vec<u64>,
    current_jump_idx:       usize,
}

impl MemoryViewer {
    pub fn new() -> Self {
        Self {
            base_address:       0x0,
            highlighted_offset: 0,
            bytes:              [0x0; READ_SIZE],
            read_successful:    false,
            changed_highlights: HashMap::new(),
            copied_highlights:  HashMap::new(),
            clipboard:          Clipboard::new().ok(),
            jump_history:       vec![0x0],
            current_jump_idx:   0,
        }
    }

    pub fn poll(&mut self) {
        let Ok(read_bytes) = read_unsafe::<[u8; READ_SIZE]>(self.base_address) else {
            self.read_successful = false;
            return;
        };

        self.read_successful = true;
        let now = Instant::now();

        for (idx, stored_byte) in self.bytes.iter_mut().enumerate() {
            let new_byte = read_bytes[idx];
            if *stored_byte != new_byte {
                *stored_byte = new_byte;

                self.changed_highlights
                    .insert(self.base_address + idx as u64, now + HIGHLIGHT_DURATION);
            }
        }
        self.cleanup_hashmaps();
    }

    fn update_bytes(&mut self) {
        if let Ok(read_bytes) = read_unsafe::<[u8; READ_SIZE]>(self.base_address) {
            self.read_successful = true;
            self.bytes = read_bytes
        } else {
            self.read_successful = false
        }
    }

    fn cleanup_hashmaps(&mut self) {
        let now = Instant::now();

        self.changed_highlights
            .retain(|_, expires_at| *expires_at > now);

        self.copied_highlights
            .retain(|_, expires_at| *expires_at > now);
    }

    pub fn increment_base(&mut self, increment: i64) {
        self.base_address = self.base_address.saturating_add_signed(increment);
        self.update_bytes();
        self.increment_highlighted(-increment)
    }

    pub fn increment_highlighted(&mut self, increment: i64) {
        self.highlighted_offset = self.highlighted_offset.saturating_add_signed(increment);

        if self.bytes.len() <= self.highlighted_offset as usize {
            self.increment_base(increment);
        }
    }

    pub fn jump(&mut self, addr: u64) {
        self.current_jump_idx += 1;
        self.base_address = addr;
        self.highlighted_offset = 0;
        self.clear_forward_history();
        self.jump_history.push(addr);
        self.update_bytes();
    }

    pub fn jump_module_relative(&mut self, addr: u64) {
        if let Some(addr) = module_base().checked_add(addr) {
            self.jump(addr)
        }
    }

    pub fn jump_relative_i32_at_highlighted(&mut self) {
        if let Ok(offset) = read_from_slice::<i32>(&self.bytes, self.highlighted_offset)
            && let Some(addr) = self.base_address.checked_add_signed(offset as i64 + 5)
        {
            self.jump(addr);
        }
    }

    pub fn jump_absolute_at_highlighted(&mut self) {
        let addr = match is_32() {
            true => {
                read_from_slice::<u32>(&self.bytes, self.highlighted_offset).map(|val| val as u64)
            }
            false => read_from_slice::<u64>(&self.bytes, self.highlighted_offset),
        };
        if let Ok(addr) = addr {
            self.jump(addr);
        }
    }

    pub fn copy_dword_at_highlighted(&mut self) {
        self.copied_highlights.clear();
        if let Ok(dword) = read_from_slice::<u32>(&self.bytes, self.highlighted_offset)
            && self.read_successful
            && let Some(clipboard) = &mut self.clipboard
            && clipboard.set_text(format!("{:#X}", dword)).is_ok()
        {
            self.add_bytes_to_copied(4);
        }
    }

    pub fn copy_qword_at_highlighted(&mut self) {
        self.copied_highlights.clear();
        if let Ok(qword) = read_from_slice::<u64>(&self.bytes, self.highlighted_offset)
            && self.read_successful
            && let Some(clipboard) = &mut self.clipboard
            && clipboard.set_text(format!("{:#X}", qword)).is_ok()
        {
            self.add_bytes_to_copied(8);
        }
    }

    pub fn copy_absolute_address_at_highlighted(&mut self) {
        self.copied_highlights.clear();
        if let Some(clipboard) = &mut self.clipboard
            && clipboard
                .set_text(format!("{:#X}", self.base_address + self.highlighted_offset))
                .is_ok()
        {
            self.add_bytes_to_copied(1);
        }
    }

    pub fn copy_relative_address_at_highlighted(&mut self) {
        self.copied_highlights.clear();
        if let Some(clipboard) = &mut self.clipboard
            && clipboard
                .set_text(format!(
                    "{:#X}",
                    (self.base_address + self.highlighted_offset).saturating_sub(module_base())
                ))
                .is_ok()
        {
            self.add_bytes_to_copied(1);
        }
    }

    pub fn write_at_highlighted<T: Pod>(&self, val: T) -> Result<(), SysError> {
        write_unsafe::<T>(self.base_address + self.highlighted_offset, val)
    }

    fn add_bytes_to_copied(&mut self, amount: usize) {
        let now = Instant::now();

        for i in 0..amount {
            self.copied_highlights.insert(
                self.base_address + self.highlighted_offset + i as u64,
                now + HIGHLIGHT_DURATION,
            );
        }
    }

    pub fn jump_forwards(&mut self) {
        let idx = self.current_jump_idx + 1;
        if idx < self.jump_history.len() {
            self.current_jump_idx = idx;
            self.base_address = self.jump_history[idx];
            self.highlighted_offset = 0;
            self.update_bytes();
        }
    }

    pub fn jump_backwards(&mut self) {
        if let Some(idx) = self.current_jump_idx.checked_sub(1)
            && idx < self.jump_history.len()
        {
            self.current_jump_idx = idx;
            self.base_address = self.jump_history[idx];
            self.highlighted_offset = 0;
            self.update_bytes();
        }
    }

    fn clear_forward_history(&mut self) {
        self.jump_history.truncate(self.current_jump_idx + 1)
    }
}
