use serde::{de, Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;

static ROOT_PATH: &'static str = "svdjson/";

#[derive(Serialize, Deserialize, Debug)]
pub enum Operator {
    #[serde(rename = "include")]
    Include,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Op {
    #[serde(rename = "type")]
    t: Option<Operator>,
    file: Option<String>,
    #[serde(default)]
    pub args: HashMap<String, String>,
}

impl Op {
    pub fn load<'a, T>(&'a self) -> T
    where
        T: de::DeserializeOwned,
    {
        let path = ROOT_PATH.to_string() + self.file.as_ref().unwrap().as_str();
        let file = File::open(path).unwrap();
        let t: T = serde_json::from_reader(file).unwrap();
        t
    }
}
