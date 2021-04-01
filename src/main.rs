mod field;
mod mode;
mod peripheral;
mod register;
mod utils;

use peripheral::PeripheralCsv;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut rdr = csv::Reader::from_path("./csvs/index.csv").unwrap();
    for r in rdr.deserialize() {
        let pcsv: PeripheralCsv = r.unwrap();
        let p = pcsv.to_peripheral();
        let json = serde_json::to_string_pretty(&p).unwrap();
        let path = String::from("./json/") + &p.name + ".json";
        let mut f = File::create(path).unwrap();
        f.write(json.as_bytes()).unwrap();
    }
}
