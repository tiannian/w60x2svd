use crate::mode::AccessMode;
use crate::op;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    #[serde(flatten)]
    pub op: op::Op,
    pub name: Option<String>,   // Notice: This file is required in svd file.
    pub offset: Option<String>, // Notice: This file is required in svd file.
    pub description: Option<String>,
    pub mode: Option<AccessMode>,
    pub reset: Option<String>,
    pub fields: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Register;
    use std::fs::File;
    #[test]
    fn load_json() {
        let file = File::open("svdjson/dma/registers.json").unwrap();
        let registers: Vec<Register> = serde_json::from_reader(file).unwrap();
        println!("{:?}", registers);
    }
}
