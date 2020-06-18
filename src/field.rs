use crate::dim::Dim;
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
    pub dim_file: Option<String>,
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

        let field_info = FieldInfoBuilder::default()
            .name(self.name)
            .description(self.description)
            .access(access)
            .bit_range(bit_range)
            .build()
            .unwrap();

        if let Some(p) = self.dim_file {
            let dim = Dim::load(&p).get_svd();
            SvdField::Array(field_info, dim)
        } else {
            SvdField::Single(field_info)
        }
    }
}
