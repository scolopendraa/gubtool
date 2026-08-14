use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct ApplyAttachError {
    pub error_count: usize,
}

impl Error for ApplyAttachError {}

impl Display for ApplyAttachError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} error(s) occurred while applying attach preferences", self.error_count)
    }
}
