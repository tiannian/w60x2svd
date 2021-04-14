use crate::field::FieldCsv;
use crate::register::{Register, RegisterCsv};
use crate::utils;
use serde::{Deserialize, Serialize};
use std::path;
use svd_parser::svd::peripheral::{Peripheral as SvdPeripheral, PeripheralBuilder};
use svd_parser::svd::AddressBlock as SvdAddressBlock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Usage {
    Registers,
    Buffer,
    Resverd,
}

impl From<Usage> for String {
    fn from(ab: Usage) -> Self {
        match ab {
            Usage::Registers => String::from("Registers"),
            Usage::Buffer => String::from("Buffer"),
            Usage::Resverd => String::from("Resverd"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddressBlock {
    pub offset: u32,
    pub size: u32,
    pub usage: Usage,
}

impl From<AddressBlock> for SvdAddressBlock {
    fn from(ab: AddressBlock) -> Self {
        Self {
            offset: ab.offset,
            size: ab.size,
            usage: ab.usage.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeripheralCsv {
    pub name: String,
    pub begin: String,
    pub end: String,
    pub description: String,
}

impl PeripheralCsv {
    pub fn to_peripheral(self) -> Peripheral {
        let base_address = utils::from_radix_to_u32(&self.begin).unwrap();
        let end_address = utils::from_radix_to_u32(&self.end).unwrap();
        let offset = 0;
        let size = end_address - base_address + 1;
        let mut p = Peripheral {
            name: self.name,
            version: String::from("0.1"),
            description: self.description,
            base_address,
            address_block: AddressBlock {
                offset,
                size,
                usage: Usage::Registers,
            },
            registers: Vec::new(),
        };
        p.read_csv();
        p
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Peripheral {
    pub name: String,
    pub version: String,
    pub description: String,
    pub base_address: u32,
    pub address_block: AddressBlock,
    pub registers: Vec<Register>,
}

impl Peripheral {
    pub fn read_csv(&mut self) {
        let path = String::from("./csvs/") + &self.name + ".csv";
        let mut rdr = csv::Reader::from_path(path).unwrap();
        for r in rdr.deserialize() {
            let record: RegisterCsv = r.unwrap();
            let mut reg = record.to_register();
            let p = format!("./csvs/{}/{}.csv", &self.name, &reg.name.to_uppercase());
            let path = path::Path::new(&p);
            if path.exists() {
                let mut rdr = csv::Reader::from_path(path).unwrap();
                for f in rdr.deserialize() {
                    let field_csv: FieldCsv = f.unwrap();
                    let field = field_csv.to_field();
                    reg.fields.push(field);
                }
            }
            self.registers.push(reg);
        }
    }

    pub fn to_svd(self) -> SvdPeripheral {
        let builder = PeripheralBuilder::default();
        let mut registers = Vec::new();
        for register in self.registers {
            registers.push(register.to_svd())
        }
        builder
            .name(self.name)
            .base_address(self.base_address.into())
            .version(Some(self.version))
            .description(Some(self.description))
            .address_block(Some(self.address_block.into()))
            .registers(Some(registers))
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_csv_de() {
        let mut p = Peripheral {
            name: String::from("timer"),
            version: String::from("1.0"),
            description: String::from(""),
            base_address: 0,
            address_block: AddressBlock {
                offset: 0,
                size: 0,
                usage: Usage::Resverd,
            },
            registers: Vec::new(),
        };
        p.read_csv();
        println!("{:#?}", p);
    }
}
