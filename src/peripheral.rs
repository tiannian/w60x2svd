use crate::op;
use crate::register::Register;
use serde::{Deserialize, Serialize};
use svd_parser::{
    addressblock::AddressBlock, peripheral::PeripheralBuilder, Peripheral as SvdPeripheral,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Peripheral {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub base_address: String,
    pub offset: String,
    pub length: String,
    pub registers: Option<op::Op>,
    pub interrupt: Option<op::Op>,
}

impl Peripheral {
    pub fn get_svd(self) -> SvdPeripheral {
        let mut builder = PeripheralBuilder::default();
        let name = self.name;
        builder = builder.name(name);

        // peripheral address.
        // TODO: add test for hex str.
        let base_address = u32::from_str_radix(&self.base_address[2..], 16).unwrap();
        builder = builder.base_address(base_address);

        // peripheral address block.
        // TODO: offset, length,must be Some.
        let offset = u32::from_str_radix(&self.offset[2..], 16).unwrap();
        let size = u32::from_str_radix(&self.length[2..], 16).unwrap();
        let address_block = AddressBlock {
            offset,
            size,
            usage: "registers".to_string(),
        };
        builder = builder.address_block(Some(address_block));

        builder = builder.description(self.description);

        let registers = match self.registers {
            Some(v) => {
                let rs: Vec<Register> = v.load();

                let mut registers = Vec::new();

                for register in rs {
                    let r = register.get_svd();
                    registers.push(r);
                }

                Some(registers)
            }
            None => None,
        };
        builder = builder.registers(registers);

        // create peripheral.
        builder.build().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Peripheral;
    use std::fs::File;
    #[test]
    fn load_json() {
        let file = File::open("svdjson/clk/peripheral.json").unwrap();
        let registers: Peripheral = serde_json::from_reader(file).unwrap();
        println!("{:?}", registers);
    }
}
