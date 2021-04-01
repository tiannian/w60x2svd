use crate::mode::AccessMode;
use crate::utils;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldCsv {
    #[serde(rename = "位")]
    pub bit: String,
    #[serde(rename = "访问")]
    pub access: String,
    #[serde(rename = "操作说明")]
    pub description: String,
    #[serde(rename = "复位值")]
    pub reset: String,
}

impl FieldCsv {
    pub fn to_field(self) -> Field {
        let re = Regex::new(r"\[(\d{0,2})(\s?:\s?\d{0,2})?\]").unwrap();
        let caps = re.captures(&self.bit).unwrap();
        let begin = caps.get(1).unwrap().as_str();
        let offset: u32 = begin.parse().unwrap();
        let width = if let Some(end) = caps.get(2) {
            let end_str = &end.as_str().replace(":", "").replace(" ", "");
            let end: u32 = end_str.parse().unwrap();
            offset - end
        } else {
            1
        };
        let access = utils::from_string_to_access(&self.access);
        let re = Regex::new(r"([a-z0-9_A-Z]*)\n?([\s\S]*)").unwrap();
        let caps = re.captures(&self.description).unwrap();
        let name = caps.get(1).unwrap().as_str().to_lowercase();
        let other = caps.get(2).unwrap().as_str();
        let splited_desc = other.split('\n');
        let mut enumerated_value = EnumeratedValue {
            name: name.clone(),
            usage: access.clone(),
            values: Vec::new(),
        };

        let mut mdescription = String::new();

        for desc in splited_desc {
            let re = Regex::new(r"\d’[bh](\d+)([\s\S]*)").unwrap();
            if let Some(caps) = re.captures(desc) {
                let value_str = caps.get(1).unwrap().as_str();
                let description = caps.get(2).unwrap().as_str().replace("：", "");
                let value: u32 = value_str.parse().unwrap();
                let v = Value {
                    name: String::from(name.clone()) + "_" + value_str,
                    description: Some(String::from(description.trim())),
                    value,
                    default: false,
                };
                enumerated_value.values.push(v);
            } else {
                mdescription += desc
            }
        }

        if enumerated_value.values.len() == 0 {
            Field {
                name,
                offset,
                width,
                access,
                description: Some(mdescription),
                enumerated_value: None,
            }
        } else {
            Field {
                name,
                offset,
                width,
                access,
                description: Some(mdescription),
                enumerated_value: Some(enumerated_value),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub description: Option<String>,
    pub value: u32,
    pub default: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumeratedValue {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub usage: Option<AccessMode>,
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Field {
    pub name: String,
    pub offset: u32,
    pub width: u32,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub access: Option<AccessMode>,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub enumerated_value: Option<EnumeratedValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_csv_de() {
        let mut rdr = csv::Reader::from_path("./csvs/timer/I2S_CLK_CTRL.csv").unwrap();
        for r in rdr.deserialize() {
            let record: FieldCsv = r.unwrap();
            let _field = record.to_field();
        }
    }
}
