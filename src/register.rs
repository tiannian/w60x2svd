use crate::field::Field;
use crate::mode::AccessMode;
use crate::utils;
use serde::{Deserialize, Serialize};
use svd_parser::svd::{
    registerinfo::RegisterInfoBuilder, Register as SvdRegister, RegisterCluster,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RegisterCsv {
    #[serde(rename = "偏移地址")]
    pub offset: String,
    #[serde(rename = "名称")]
    pub title: String,
    #[serde(rename = "缩写")]
    pub name: String,
    #[serde(rename = "访问")]
    pub access: String,
    #[serde(rename = "描述")]
    pub description: String,
    #[serde(rename = "复位值")]
    pub reset: String,
}

impl RegisterCsv {
    pub fn to_register(self) -> Register {
        let offset = utils::from_radix_to_u32(&self.offset);
        let desc = self.description.trim();
        let description = if desc == "" {
            None
        } else {
            Some(String::from(desc.trim()))
        };
        let mode = utils::from_string_to_access(&self.access);
        let reset = utils::from_radix_to_u32(&self.reset);
        let name = self.name.to_lowercase();
        println!("register name: {}", name);
        Register {
            name,
            offset,
            description,
            mode,
            reset,
            size: Some(32),
            fields: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Register {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<AccessMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    pub fields: Vec<Field>,
}

impl Register {
    pub fn to_svd(self) -> RegisterCluster {
        let builder = RegisterInfoBuilder::default();
        let mut fields = Vec::new();
        for field in self.fields {
            let f = field.to_svd();
            fields.push(f)
        }
        println!("read register: {}", self.name);
        let mode = if let Some(m) = self.mode {
            Some(m.into())
        } else {
            None
        };
        let reset = if let Some(m) = self.reset {
            Some(m.into())
        } else {
            None
        };
        let info = builder
            .name(self.name)
            .address_offset(self.offset.unwrap())
            .description(self.description)
            .size(self.size)
            .access(mode)
            .reset_value(reset)
            .fields(Some(fields))
            .build()
            .unwrap();
        let register = SvdRegister::Single(info);
        RegisterCluster::Register(register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_csv_de() {
        let mut rdr = csv::Reader::from_path("./csvs/timer.csv").unwrap();
        for r in rdr.deserialize() {
            let record: RegisterCsv = r.unwrap();
            let _reg = record.to_register();
            // println!("{:?}", reg);
        }
    }
}
