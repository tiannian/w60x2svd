use crate::dim::Dim;
use crate::mode::AccessMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use svd_parser::{
    access::Access, registerinfo::RegisterInfoBuilder, Register as SvdRegister, RegisterCluster,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: Option<String>,   // Notice: This file is required in svd file.
    pub offset: Option<String>, // Notice: This file is required in svd file.
    pub description: Option<String>,
    pub mode: Option<AccessMode>,
    pub reset: Option<String>,
    pub fields: Option<String>,
    // Need svd_parser support.
    pub data_type: Option<String>,
    pub expressions: Option<HashMap<String, String>>,
    pub dim_file: Option<String>,
}

impl Register {
    pub fn load(path: &str) -> Vec<Self> {
        println!("load device definition.\nReading {}", path);
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub fn get_svd(self) -> RegisterCluster {
        let offset = u32::from_str_radix(&self.offset.unwrap()[2..], 16).unwrap();
        let reset_value = match self.reset {
            Some(v) => Some(u32::from_str_radix(&v[2..], 16).unwrap()),
            None => None,
        };
        let access = match self.mode {
            Some(v) => match v {
                AccessMode::Unknown => None,
                AccessMode::RW => Some(Access::ReadWrite),
                AccessMode::RO => Some(Access::ReadOnly),
                AccessMode::WO => Some(Access::WriteOnly),
            },
            None => None,
        };

        // TODO: Load fields.

        let info = RegisterInfoBuilder::default()
            .name(self.name.unwrap())
            .address_offset(offset)
            .description(self.description)
            .reset_value(reset_value)
            .access(access)
            .build()
            .unwrap();
        if let Some(dim) = self.dim_file {
            let de = Dim::load(&dim);
            RegisterCluster::Register(SvdRegister::Array(info, de.get_svd()))
        } else {
            RegisterCluster::Register(SvdRegister::Single(info))
        }
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
