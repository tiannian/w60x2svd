use crate::mode::AccessMode;
use crate::op;
use serde::{Deserialize, Serialize};
use svd_parser::{Register as SvdRegister, RegisterCluster, registerinfo::RegisterInfoBuilder, access:Access};

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

impl Register {
    pub fn get_svd(self) -> RegisterCluster {
        let offset = u32::from_str_radix(&self.offset.unwrap()[2..], 16).unwrap();
        let reset_value = match self.reset {
            Some(v) => Some(u32::from_str_radix(&v[2..], 16).unwrap()),
            None => None
        };
        let info = RegisterInfoBuilder::default()
            .name(self.name.unwrap())
            .address_offset(offset)
            .description(self.description)
            .reset_value(reset_value)
            .access(self.mode)
            .build()
            .unwrap();
        RegisterCluster::Register(SvdRegister::Single(info))
    }
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
