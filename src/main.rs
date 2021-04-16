pub mod field;
pub mod mode;
pub mod peripheral;
pub mod register;
pub mod utils;

use peripheral::PeripheralCsv;
use std::fs;
use std::fs::File;
use std::io::Write;
use svd_parser::svd::device::DeviceBuilder;
use clap::Clap;

#[derive(Clap)]
struct IO {
    #[clap(short = 'i', long = "input",)]
    pub input: String,
    #[clap(short = 'o', long = "output",)]
    pub output: String,
}

#[derive(Clap)]
enum SubCommand {
    Json(IO),
    Svd(IO),
}

fn main() {
    let opt = SubCommand::parse();
    match opt {
        SubCommand::Json(io) => json(io),
        SubCommand::Svd(io) => svd(io),
    }
}

fn json(io: IO) {
    let path = format!("{}/index.csv", io.input);
    let mut rdr = csv::Reader::from_path(&path).unwrap();
    for r in rdr.deserialize() {
        let pcsv: PeripheralCsv = r.unwrap();
        let p = pcsv.to_peripheral();
        let json = serde_json::to_string_pretty(&p).unwrap();
        let path = String::from(&io.output) + &p.name + ".json";
        let mut f = File::create(path).unwrap();
        f.write(json.as_bytes()).unwrap();
    }
}

fn svd(io: IO) {
    let builder = DeviceBuilder::default();
    let mut peripherals = Vec::new();
    for dir in fs::read_dir(&io.input).unwrap() {
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
    let mut result_file = File::create(&io.output).unwrap();
    result_file.write(svd_str.as_bytes()).unwrap();
}

