use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    args: Option<HashMap<String, String>>,
}
