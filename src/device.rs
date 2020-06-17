use serde::{Serialize, Deserialize};
use crate::op::Op;
use svd_parser::{Device as SvdDevice, device::DeviceBuilder};
use crate::peripheral::Peripheral;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    name: String,
    version: Option<String>,
    schema_version: Option<String>,
    width: Option<u32>,
    cpu: Option<String>,
    peripherals: Vec<Op>,
}

impl Device {
    pub fn get_svd(self) -> SvdDevice {
        let mut peripherals = Vec::new();
        for peripheral in self.peripherals {
            let p: Peripheral = peripheral.load();
            println!("{:?}", p);
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


