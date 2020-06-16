use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AccessMode {
    Unknown,
    RW,
    RO,
    WO,
}

impl Default for AccessMode {
    fn default() -> AccessMode {
        AccessMode::Unknown
    }
}

