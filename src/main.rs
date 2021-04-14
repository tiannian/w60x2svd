pub mod field;
pub mod mode;
pub mod peripheral;
pub mod register;
pub mod utils;

use peripheral::PeripheralCsv;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use svd_parser::svd::device::DeviceBuilder;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let command = args.next().unwrap();
    // println!("command: {}", command.unwrap());
    if command == "json" {
        let mut rdr = csv::Reader::from_path("./csvs/index.csv").unwrap();
        for r in rdr.deserialize() {
            let pcsv: PeripheralCsv = r.unwrap();
            let p = pcsv.to_peripheral();
            let json = serde_json::to_string_pretty(&p).unwrap();
            let path = String::from("./json/") + &p.name + ".json";
            let mut f = File::create(path).unwrap();
            f.write(json.as_bytes()).unwrap();
        }
    } else if command == "svd" {
        let builder = DeviceBuilder::default();
        let mut peripherals = Vec::new();
        for dir in fs::read_dir("./json").unwrap() {
            let path = dir.unwrap().path();
            let file = File::open(path).unwrap();
            let p: peripheral::Peripheral = serde_json::from_reader(file).unwrap();
            println!("read peripheral: {}", p.name);
            peripherals.push(p.to_svd());
        }
        let device = builder
            .name(String::from("w600"))
            .version(Some(String::from("v0.1")))
            .description(Some(String::from("w600")))
            .width(Some(32))
            .peripherals(peripherals)
            .build()
            .unwrap();
        let svd_str = svd_parser::encode(&device).unwrap();
        let mut result_file = File::create("w600.svd").unwrap();
        result_file.write(svd_str.as_bytes()).unwrap();
    }
}
