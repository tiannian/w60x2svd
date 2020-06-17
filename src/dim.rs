use serde::{Deserialize, Serialize};
use std::fs::File;
use svd_parser::{dimelement::DimElementBuilder, DimElement};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dim {
    pub total: u32,
    pub increment: String,
    pub index: Vec<String>,
}

impl Dim {
    pub fn load(path: &str) -> Self {
        println!("load dim definition.\nReading {}", path);
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub fn get_svd(self) -> DimElement {
        let increment = u32::from_str_radix(&self.increment[2..], 16).unwrap();
        DimElementBuilder::default()
            .dim(self.total)
            .dim_increment(increment)
            .dim_index(Some(self.index))
            .build()
            .unwrap()
    }
}
