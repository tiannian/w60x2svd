use serde::{Deserialize, Serialize};
use svd_parser::access::Access;

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

impl AccessMode {
    pub fn get_svd(self) -> Option<Access> {
        match self {
            AccessMode::Unknown => None,
            AccessMode::RW => Some(Access::ReadWrite),
            AccessMode::RO => Some(Access::ReadOnly),
            AccessMode::WO => Some(Access::WriteOnly),
        }
    }
}
