// crates/zenthra-core/src/id.rs

use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Unique widget identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub fn new() -> Self {
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub fn from_u64(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}
