use crate::mode::AccessMode;
use serde::{Deserialize, Serialize};

use std::fs::File;
use svd_parser::{
    bitrange::BitRange, bitrange::BitRangeType, field::Field as SvdField,
    fieldinfo::FieldInfoBuilder,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub offset: Option<u32>,
    pub width: Option<u32>,
    pub access: Option<AccessMode>,
    pub description: Option<String>,
    pub enumerated_value: Option<String>,
    pub dim: Option<String>,
}

impl Field {
    pub fn load(path: &str) -> Vec<Self> {
        println!("load field definition.\nReading {}", path);
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub fn get_svd(self) -> SvdField {
        let access = match self.access {
            Some(a) => a.get_svd(),
            None => None,
        };

        let bit_range = BitRange {
            offset: self.offset.unwrap(),
            width: self.width.unwrap_or(1),
            range_type: BitRangeType::BitRange,
        };

        println!("{}:{},{}", self.name, bit_range.lsb(), bit_range.msb());

        SvdField::Single(
            FieldInfoBuilder::default()
                .name(self.name)
                .description(self.description)
                .access(access)
                .bit_range(bit_range)
                .build()
                .unwrap(),
        )
    }
}
