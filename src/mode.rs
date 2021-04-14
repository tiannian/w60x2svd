use serde::{Deserialize, Serialize};
use svd_parser::access::Access;
use svd_parser::svd::usage::Usage;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl From<AccessMode> for Usage {
    fn from(a: AccessMode) -> Self {
        match a {
            AccessMode::RO => Usage::Read,
            AccessMode::WO => Usage::Write,
            AccessMode::RW => Usage::ReadWrite,
            AccessMode::Unknown => panic!("unknown"),
        }
    }
}

impl From<AccessMode> for Access {
    fn from(a: AccessMode) -> Self {
        match a {
            AccessMode::RO => Access::ReadOnly,
            AccessMode::WO => Access::WriteOnce,
            AccessMode::RW => Access::ReadWrite,
            AccessMode::Unknown => panic!("unknown"),
        }
    }
}

// impl AccessMode {
// pub fn get_svd(self) -> Option<Access> {
//     match self {
//         AccessMode::Unknown => None,
//         AccessMode::RW => Some(Access::ReadWrite),
//         AccessMode::RO => Some(Access::ReadOnly),
//         AccessMode::WO => Some(Access::WriteOnly),
//     }
// }
// }
