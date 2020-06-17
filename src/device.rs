use crate::op::Op;
use crate::peripheral::Peripheral;
use serde::{Deserialize, Serialize};
use svd_parser::{device::DeviceBuilder, Device as SvdDevice};

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
