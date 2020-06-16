use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Peripheral {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub base_address: String,
    pub offset: String,
    pub length: Option<String>,
    pub registers: Option<String>,
    pub interrupt: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::Peripheral;
    use std::fs::File;
    #[test]
    fn load_json() {
        let file = File::open("svdjson/clk/registers.json").unwrap();
        let registers: Vec<Peripheral> = serde_json::from_reader(file).unwrap();
        println!("{:?}", registers);
    }
}
