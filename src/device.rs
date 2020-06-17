use crate::peripheral::Peripheral;
use serde::{Deserialize, Serialize};
use std::fs::File;
use svd_parser::{device::DeviceBuilder, Device as SvdDevice};

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    name: String,
    version: Option<String>,
    schema_version: Option<String>,
    width: Option<u32>,
    cpu: Option<String>,
    peripherals_files: Vec<String>,
}

impl Device {
    pub fn load(path: &str) -> Self {
        println!("load device definition\nReading {}", path);
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub fn get_svd(self) -> SvdDevice {
        let mut peripherals = Vec::new();
        for peripheral_path in self.peripherals_files {
            let p = Peripheral::load(&peripheral_path);
            peripherals.push(p.get_svd());
        }
        DeviceBuilder::default()
            .name(self.name)
            .version(self.version)
            .schema_version(self.schema_version)
            .width(self.width)
            .peripherals(peripherals)
            .build()
            .unwrap()
    }
}
