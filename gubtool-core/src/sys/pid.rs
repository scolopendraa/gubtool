use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pid(u32);

impl Pid {
    pub fn new(pid: u32) -> Self {
        Self(pid)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    #[cfg(unix)]
    pub fn as_nix(&self) -> nix::unistd::Pid {
        nix::unistd::Pid::from_raw(self.0 as i32)
    }
}

impl Display for Pid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
