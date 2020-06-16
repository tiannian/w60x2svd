use crate::mode::AccessMode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    name: Option<String>,
    description: Option<String>,
    values: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumeratedValue {
    name: Option<String>,
    usage: Option<AccessMode>,
    values: Vec<Value>,
}
