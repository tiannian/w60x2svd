mod device;
mod enumerated_values;
mod field;
mod mode;
mod op;
mod peripheral;
mod register;

use device::Device;
use std::fs::File;
use std::io::BufWriter;
use svd_parser::encode::Encode;

fn main() {
    let file = File::open("svdjson/devices.json").unwrap();
    let device: Device = serde_json::from_reader(file).unwrap();

    let d = device.get_svd();

    let file = File::create("svd/w600.base.svd").unwrap();
    let f = BufWriter::new(file);
    d.encode().unwrap().write(f).unwrap();
}
