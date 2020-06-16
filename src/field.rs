use crate::mode::AccessMode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub bit: Option<u64>,
    pub access: Option<AccessMode>,
    pub description: Option<String>,
    pub enumerated_value: Option<String>,
}
